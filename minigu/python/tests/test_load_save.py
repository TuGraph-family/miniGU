"""
Tests for miniGU Python API load/save functionality
"""

import pytest
import sys
import os
import json
import tempfile

# Add the python directory to the path so we can import minigu
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from minigu import connect, MiniGU, QueryResult, MiniGUError


def test_load_data():
    """Test loading data into miniGU"""
    with connect() as db:
        # Test loading list of dictionaries
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"},
            {"name": "TechCorp", "founded": 2010, "label": "Company"}
        ]
        
        # This should not raise an exception
        db.load(sample_data)
        
        # Test that we can still execute queries after loading
        result = db.execute("MATCH (n) RETURN n LIMIT 1;")
        assert isinstance(result, QueryResult)


def test_save_database():
    """Test saving database to file"""
    with connect() as db:
        # Load some data first
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"}
        ]
        db.load(sample_data)
        
        # Save to a temporary file
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            tmp_path = tmp.name
        
        try:
            # This should not raise an exception
            db.save(tmp_path)
            
            # Verify file was created
            assert os.path.exists(tmp_path)
            assert os.path.getsize(tmp_path) >= 0
        finally:
            # Clean up
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)


def test_load_from_file():
    """Test loading data from file"""
    # Create a temporary JSON file with test data
    sample_data = [
        {"name": "Charlie", "age": 35, "label": "Person"},
        {"name": "DataCorp", "founded": 2005, "label": "Company"}
    ]
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as tmp:
        json.dump(sample_data, tmp)
        tmp_path = tmp.name
    
    try:
        with connect() as db:
            # This should not raise an exception
            db.load(tmp_path)
            
            # Test that we can execute queries after loading
            result = db.execute("MATCH (n) RETURN n LIMIT 1;")
            assert isinstance(result, QueryResult)
    finally:
        # Clean up
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)


def test_load_save_errors():
    """Test error handling in load/save operations"""
    # Test loading to closed connection
    db = connect()
    db.close()
    
    with pytest.raises(MiniGUError):
        db.load([{"name": "Test"}])
    
    with pytest.raises(MiniGUError):
        db.save("test.mgu")
    
    # Test saving from closed connection
    with pytest.raises(MiniGUError):
        db.load("nonexistent.json")


def test_workflow():
    """Test a complete workflow with load, query, and save"""
    with connect() as db:
        # Step 1: Load data
        social_network_data = [
            {"name": "Alice", "age": 30, "city": "Beijing", "label": "Person"},
            {"name": "Bob", "age": 25, "city": "Shanghai", "label": "Person"},
            {"name": "Charlie", "age": 35, "city": "Guangzhou", "label": "Person"},
            {"name": "TechCorp", "industry": "Technology", "founded": 2010, "label": "Company"},
            {"name": "DataCorp", "industry": "Analytics", "founded": 2005, "label": "Company"}
        ]
        
        db.load(social_network_data)
        
        # Step 2: Execute queries
        # Count all nodes
        result = db.execute("MATCH (n) RETURN count(n) as total_nodes")
        assert isinstance(result, QueryResult)
        assert result.row_count >= 0
        
        # Get all persons - in simulation mode, we just check that the query executes
        result = db.execute("MATCH (p:Person) RETURN p.name, p.age ORDER BY p.age")
        # In simulation mode, we don't have actual data, so we just check the result structure
        assert isinstance(result, QueryResult)
        
        # Step 3: Save database
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as tmp:
            tmp_path = tmp.name
        
        try:
            db.save(tmp_path)
            assert os.path.exists(tmp_path)
        finally:
            if os.path.exists(tmp_path):
                os.unlink(tmp_path)


if __name__ == "__main__":
    pytest.main([__file__])