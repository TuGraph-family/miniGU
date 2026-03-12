#!/bin/bash
set -eu
set -o pipefail

workspace=$(realpath $(dirname $0)/../../)
basedir=$(dirname $(realpath $0))
scale_factors=(0.1 0.3 1 3 10 30 100 300 1000)

duckdb=$workspace/tools/duckdb
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
                new_path=$dir/"$type".csv
                $duckdb -c "copy (select id from '$path' where type = '$type' order by id) to '$new_path' (header, delimiter ',')"
            done
            ;;
        "Place")
            for type in City Country Continent; do
                new_path=$dir/"$type".csv
                $duckdb -c "copy (select id from '$path' where type = '$type' order by id) to '$new_path' (header, delimiter ',')"
            done
            ;;
        *)
            new_path=$dir/"$vertex".csv
            $duckdb -c "copy (select id from '$path' order by id) to '$new_path' (header, delimiter ',')"
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
            sql1="select Place1Id src, Place2Id dst from '$path' e, '$place' s where Place1Id = s.id and s.type = 'Country' order by Place1Id, Place2Id"
            sql2="select Place1Id src, Place2Id dst from '$path' e, '$place' s where Place1Id = s.id and s.type = 'City' order by Place1Id, Place2Id"
            new_path1=$dir/Country_isPartOf_Continent.csv
            new_path2=$dir/City_isPartOf_Country.csv
            $duckdb -c "copy ($sql1) to '$new_path1' (header, delimiter ',')"
            $duckdb -c "copy ($sql2) to '$new_path2' (header, delimiter ',')"
            ;;
        "Organisation_isLocatedIn_Place")
            organisation="$dir/Organisation/part-*.parquet"
            sql1="select OrganisationId src, PlaceId dst from '$path' e, '$organisation' s where OrganisationId = s.id and s.type = 'Company' order by OrganisationId, PlaceId"
            sql2="select OrganisationId src, PlaceId dst from '$path' e, '$organisation' s where OrganisationId = s.id and s.type = 'University' order by OrganisationId, PlaceId"
            new_path1=$dir/Company_isLocatedIn_Country.csv
            new_path2=$dir/University_isLocatedIn_City.csv
            $duckdb -c "copy ($sql1) to '$new_path1' (header, delimiter ',')"
            $duckdb -c "copy ($sql2) to '$new_path2' (header, delimiter ',')"
            ;;
        *)
            new_path=$dir/"$edge".csv
            $duckdb -c "copy (select $src src, $dst dst from '$path' order by $src, $dst) to '$new_path' (header, delimiter ',')"
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
    csv_path=$dir/Person_knows_Person.csv
    tmp_csv_path=$dir/Person_knows_Person.csv.tmp
    sql="with knows as (select src, dst from read_csv('$csv_path')) select src, dst from knows union select dst, src from knows"
    $duckdb -c "copy ($sql) to '$tmp_csv_path' (header, delimiter ',')"
    mv $tmp_csv_path $csv_path
}

for sf in ${scale_factors[@]}; do
    if ! [ -d $basedir/sf$sf ] || [ -s "$(find $basedir/sf$sf -name *.parquet -type f)" ]; then
        continue
    fi
    process_vertex $sf
    process_edge $sf
    delete_vertex $sf
    delete_edge $sf
    unique_vid $sf
    process_knows $sf
done
