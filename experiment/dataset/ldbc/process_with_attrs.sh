#!/bin/bash
set -eu
set -o pipefail

workspace=$(realpath $(dirname $0)/../../)
basedir=$(dirname $(realpath $0))
scale_factors=(0.1 0.3 1 3 10 30 100 300 1000)

duckdb=$workspace/duckdb

function to_lower() {
    echo "$1" | tr '[:upper:]' '[:lower:]'
}
vertices=(
    Comment
    Forum
    Organisation
    Person
    Place
    Post
    Tag
    TagClass
)

edges=(
    "Person_workAt_Company,PersonId,CompanyId"
    "Comment_hasTag_Tag,CommentId,TagId"
    "Comment_replyOf_Comment,Comment1Id,Comment2Id"
    "Tag_hasType_TagClass,TagId,TagClassId"
    "Place_isPartOf_Place,_,_"
    "Post_hasCreator_Person,PostId,PersonId"
    "Forum_hasTag_Tag,ForumId,TagId"
    "Forum_hasMember_Person,ForumId,PersonId"
    "Forum_containerOf_Post,ForumId,PostId"
    "Person_studyAt_University,PersonId,UniversityId"
    "Post_isLocatedIn_Country,PostId,CountryId"
    "Comment_isLocatedIn_Country,CommentId,CountryId"
    "Comment_hasCreator_Person,CommentId,PersonId"
    "Person_knows_Person,Person1Id,Person2Id"
    "Comment_replyOf_Post,CommentId,PostId"
    "TagClass_isSubclassOf_TagClass,TagClass1Id,TagClass2Id"
    "Person_likes_Post,PersonId,PostId"
    "Person_hasInterest_Tag,PersonId,TagId"
    "Post_hasTag_Tag,PostId,TagId"
    "Forum_hasModerator_Person,ForumId,PersonId"
    "Person_likes_Comment,PersonId,CommentId"
    "Person_isLocatedIn_City,PersonId,CityId"
    "Organisation_isLocatedIn_Place,_,_"
)

function process_vertex() {
    dir=$basedir/sf$1

    for vertex in ${vertices[@]}; do
        path="$dir/$vertex/part-*.parquet"
        case $vertex in
        "Organisation")
            for type in University Company; do
                new_path=$dir/$(to_lower "$type").csv
                # id 列放在最前面，其他列跟在后面
                $duckdb -c "copy (select id, * EXCLUDE (id) from '$path' where type = '$type' order by id) to '$new_path' (header, delimiter ',')"
            done
            ;;
        "Place")
            for type in City Country Continent; do
                new_path=$dir/$(to_lower "$type").csv
                # id 列放在最前面，其他列跟在后面
                $duckdb -c "copy (select id, * EXCLUDE (id) from '$path' where type = '$type' order by id) to '$new_path' (header, delimiter ',')"
            done
            ;;
        *)
            new_path=$dir/$(to_lower "$vertex").csv
            # id 列放在最前面，其他列跟在后面
            $duckdb -c "copy (select id, * EXCLUDE (id) from '$path' order by id) to '$new_path' (header, delimiter ',')"
            ;;
        esac
    done
}

function process_edge() {
    dir=$basedir/sf$1

    for edge_src_dst in ${edges[@]}; do
        IFS=',' read -r edge src dst <<<"$edge_src_dst"
        path="$dir/$edge/part-*.parquet"
        case $edge in
        "Place_isPartOf_Place")
            place="$dir/Place/part-*.parquet"
            # src 和 dst 放在最前面，其他列跟在后面
            sql1="select e.Place1Id as src, e.Place2Id as dst, e.* EXCLUDE (Place1Id, Place2Id) 
                  from '$path' e, '$place' s 
                  where e.Place1Id = s.id and s.type = 'Country' 
                  order by e.Place1Id, e.Place2Id"
            sql2="select e.Place1Id as src, e.Place2Id as dst, e.* EXCLUDE (Place1Id, Place2Id) 
                  from '$path' e, '$place' s 
                  where e.Place1Id = s.id and s.type = 'City' 
                  order by e.Place1Id, e.Place2Id"
            new_path1=$dir/country_ispartof_continent.csv
            new_path2=$dir/city_ispartof_country.csv
            $duckdb -c "copy ($sql1) to '$new_path1' (header, delimiter ',')"
            $duckdb -c "copy ($sql2) to '$new_path2' (header, delimiter ',')"
            ;;
        "Organisation_isLocatedIn_Place")
            organisation="$dir/Organisation/part-*.parquet"
            # src 和 dst 放在最前面，其他列跟在后面
            sql1="select e.OrganisationId as src, e.PlaceId as dst, e.* EXCLUDE (OrganisationId, PlaceId) 
                  from '$path' e, '$organisation' s 
                  where e.OrganisationId = s.id and s.type = 'Company' 
                  order by e.OrganisationId, e.PlaceId"
            sql2="select e.OrganisationId as src, e.PlaceId as dst, e.* EXCLUDE (OrganisationId, PlaceId) 
                  from '$path' e, '$organisation' s 
                  where e.OrganisationId = s.id and s.type = 'University' 
                  order by e.OrganisationId, e.PlaceId"
            new_path1=$dir/company_islocatedin_country.csv
            new_path2=$dir/university_islocatedin_city.csv
            $duckdb -c "copy ($sql1) to '$new_path1' (header, delimiter ',')"
            $duckdb -c "copy ($sql2) to '$new_path2' (header, delimiter ',')"
            ;;
        *)
            new_path=$dir/$(to_lower "$edge").csv
            # src 和 dst 放在最前面，其他列跟在后面
            $duckdb -c "copy (select $src as src, $dst as dst, * EXCLUDE ($src, $dst) from '$path' order by $src, $dst) to '$new_path' (header, delimiter ',')"
            ;;
        esac
    done
}

function delete_vertex() {
    dir=$basedir/sf$1

    for vertex in ${vertices[@]}; do
        rm -r $dir/$vertex
    done
}

function delete_edge() {
    dir=$basedir/sf$1

    for edge_src_dst in ${edges[@]}; do
        IFS=',' read -r edge src dst <<<"$edge_src_dst"
        rm -r $dir/$edge
    done
}

function unique_vid() {
    dir=$basedir/sf$1
    schema=$workspace/schemas/ldbc/ldbc_pathce_schema.json
    $workspace/tools/unique_vid.py -s $schema -d $dir
}

function process_knows() {
    dir=$basedir/sf$1
    csv_path=$dir/person_knows_person.csv
    tmp_csv_path=$dir/person_knows_person.csv.tmp
    # 保留所有列，同时处理双向关系
    # src 和 dst 放在最前面
    sql="with knows as (select * from read_csv('$csv_path')) 
         select src, dst, * EXCLUDE (src, dst) from knows 
         union 
         select dst as src, src as dst, * EXCLUDE (src, dst) from knows"
    $duckdb -c "copy ($sql) to '$tmp_csv_path' (header, delimiter ',')"
    mv $tmp_csv_path $csv_path
}

minigu=$workspace/../target/release/minigu

function load_to_minigu() {
    dir=$basedir/sf$1
    db_path=$dir/minigu_db
    manifest_path=$dir/manifest.json
    load_script=$dir/_load.gql

    # Skip if database already exists
    if [ -d "$db_path" ]; then
        echo "skip sf$1: $db_path already exists"
        return
    fi

    # Generate load script:
    #   1. build_ldbc_manifest: scan CSV dir -> manifest.json
    #   2. import_graph: load CSV via manifest into minigu (persisted to db_path via --path)
    cat > "$load_script" <<EOGQL
call build_ldbc_manifest("$dir", "$manifest_path")
call import_graph("ldbc", "$manifest_path")
EOGQL

    echo "loading sf$1 into $db_path ..."
    "$minigu" execute "$load_script" --path "$db_path"
    rm -f "$load_script"
    echo "done sf$1"
}

for sf in ${scale_factors[@]}; do
    if ! [ -d $basedir/sf$sf ] || [ -s "$(find $basedir/sf$sf -name *.parquet -type f)" ]; then
        continue
    fi
    process_vertex $sf
    process_edge $sf
    delete_vertex $sf
    delete_edge $sf
    # unique_vid $sf
    process_knows $sf
    load_to_minigu $sf
done

