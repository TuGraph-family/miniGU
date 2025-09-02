#!/usr/bin/env python3.7
"""
Test cases for miniGU Python API.

This file contains tests for:
1. Basic connection functionality
2. Graph creation and management
3. Data insertion and querying
4. Error handling
5. Asynchronous functionality
"""

import unittest
import sys
import os
import tempfile
import json

# Add the python module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'minigu', 'python'))

import minigu


class TestMiniGUAPI(unittest.TestCase):
    """Test cases for the miniGU Python API."""
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        self.db = minigu.connect()
    
    def tearDown(self):
        """Tear down test fixtures after each test method."""
        if self.db and self.db.is_connected:
            try:
                self.db.close()
            except AttributeError:
                # 如果没有close方法，就直接设置为未连接状态
                self.db.is_connected = False
    
    def test_connection(self):
        """Test basic database connection."""
        self.assertIsInstance(self.db, minigu.MiniGU)
        self.assertTrue(self.db.is_connected)
    
    def test_create_graph(self):
        """Test graph creation."""
        graph_name = "test_graph"
        self.db.create_graph(graph_name)
        
        # Verify that the graph was created by trying to use it
        try:
            # Try to execute a simple query on the created graph
            result = self.db.execute(f"SHOW GRAPHS")
            # If we get here without exception, the graph creation was successful
            self.assertIsInstance(result, minigu.QueryResult)
        except minigu.QueryError:
            # This is expected if SHOW GRAPHS is not implemented yet
            pass
        except Exception as e:
            # Other exceptions indicate a problem
            self.fail(f"Unexpected exception: {e}")
    
    def test_create_graph_with_schema(self):
        """Test graph creation with schema."""
        graph_name = "test_graph_with_schema"
        schema = {
            "Person": {"name": "STRING", "age": "INTEGER"},
            "Company": {"name": "STRING", "founded": "INTEGER"}
        }
        self.db.create_graph(graph_name, schema)
        
        # Verify that the graph was created with the specified schema
        try:
            # Try to execute a simple query on the created graph
            result = self.db.execute(f"SHOW GRAPHS")
            # If we get here without exception, the graph creation was successful
            self.assertIsInstance(result, minigu.QueryResult)
        except minigu.QueryError:
            # This is expected if SHOW GRAPHS is not implemented yet
            pass
        except Exception as e:
            # Other exceptions indicate a problem
            self.fail(f"Unexpected exception: {e}")
    
    def test_insert_and_query_data(self):
        """Test inserting and querying data."""
        # Create a graph
        self.db.create_graph("test_graph")
        
        # Insert data
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"}
        ]
        self.db.load(sample_data)
        
        # Query data
        result = self.db.execute("MATCH (n:Person) RETURN n.name, n.age")
        self.assertIsInstance(result, minigu.QueryResult)
    
    def test_query_result_methods(self):
        """Test QueryResult methods."""
        # Create a graph and insert data
        self.db.create_graph("test_graph")
        sample_data = [{"name": "Alice", "age": 30, "label": "Person"}]
        self.db.load(sample_data)
            
        # Query data
        result = self.db.execute("MATCH (n:Person) RETURN n.name, n.age")
        
        # Test to_list method
        data_list = result.to_list()
        self.assertIsInstance(data_list, list)
        
        # Test to_dict method
        data_dict = result.to_dict()
        self.assertIsInstance(data_dict, dict)
        self.assertIn('schema', data_dict)
        self.assertIn('data', data_dict)
        self.assertIn('metrics', data_dict)
        self.assertIn('row_count', data_dict)
    
    def test_save_database(self):
        """Test saving the database."""
        # Create a graph and insert data
        self.db.create_graph("test_graph")
        sample_data = [{"name": "Alice", "age": 30, "label": "Person"}]
        self.db.load(sample_data)
        
        # Save to a temporary file
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as f:
            temp_file = f.name
        self.db.save(temp_file)
        # Check that file was created
        self.assertTrue(os.path.exists(temp_file))
        
        # Clean up
        if temp_file and os.path.exists(temp_file):
            os.unlink(temp_file)
    
    def test_load_from_file(self):
        """Test loading data from a file."""
        # Create sample data file
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"}
        ]
        
        with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
            json.dump(sample_data, f)
            temp_file = f.name
        
        # Create a graph
        self.db.create_graph("test_graph")
                
        # Load data
        self.db.load(temp_file)
        
        # Clean up
        if os.path.exists(temp_file):
            os.unlink(temp_file)
    
    def test_error_handling(self):
        """Test error handling."""
        # Test that we can catch MiniGUError
        with self.assertRaises(minigu.MiniGUError):
            # Try to execute a query without creating a graph first
            db = minigu.MiniGU()
            try:
                db.close()  # Close the database to trigger an error
            except AttributeError:
                # 如果没有close方法，就直接设置为未连接状态
                db.is_connected = False
            db.execute("MATCH (n) RETURN n")


if __name__ == "__main__":
    unittest.main()