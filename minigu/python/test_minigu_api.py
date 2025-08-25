#!/usr/bin/env python3
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
import asyncio
import tempfile
import json

# Add the python module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'minigu', 'python'))

from minigu import connect, async_connect, MiniGU, AsyncMiniGU, MiniGUError, QueryResult

class TestMiniGUAPI(unittest.TestCase):
    """Test cases for the miniGU Python API."""
    
    def setUp(self):
        """Set up test fixtures before each test method."""
        self.db = connect()
    
    def tearDown(self):
        """Tear down test fixtures after each test method."""
        if self.db and self.db.is_connected:
            self.db.close()
    
    def test_connection(self):
        """Test basic database connection."""
        self.assertIsInstance(self.db, MiniGU)
        self.assertTrue(self.db.is_connected)
    
    def test_create_graph(self):
        """Test graph creation."""
        self.db.create_graph("test_graph")
        # If no exception is raised, the test passes
    
    def test_create_graph_with_schema(self):
        """Test graph creation with schema."""
        schema = {
            "Person": {"name": "STRING", "age": "INTEGER"},
            "Company": {"name": "STRING", "founded": "INTEGER"}
        }
        self.db.create_graph("test_graph_with_schema", schema)
        # If no exception is raised, the test passes
    
    def test_insert_and_query_data(self):
        """Test inserting and querying data."""
        # Create a graph
        self.db.create_graph("test_graph")
        
        # Insert data
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"}
        ]
        self.db.insert(sample_data)
        
        # Query data
        result = self.db.execute("MATCH (n:Person) RETURN n.name, n.age")
        self.assertIsInstance(result, QueryResult)
        self.assertEqual(result.row_count, 2)
        
        # Check data content
        data_list = result.to_list()
        self.assertEqual(len(data_list), 2)
        names = [row['n.name'] for row in data_list]
        self.assertIn("Alice", names)
        self.assertIn("Bob", names)
    
    def test_query_result_methods(self):
        """Test QueryResult methods."""
        # Create a graph and insert data
        self.db.create_graph("test_graph")
        sample_data = [{"name": "Alice", "age": 30, "label": "Person"}]
        self.db.insert(sample_data)
        
        # Query data
        result = self.db.execute("MATCH (n:Person) RETURN n.name, n.age")
        
        # Test to_list method
        data_list = result.to_list()
        self.assertIsInstance(data_list, list)
        self.assertEqual(len(data_list), 1)
        self.assertIn('n.name', data_list[0])
        self.assertIn('n.age', data_list[0])
        
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
        self.db.insert(sample_data)
        
        # Save to a temporary file
        with tempfile.NamedTemporaryFile(suffix='.mgu', delete=False) as f:
            temp_file = f.name
        
        try:
            self.db.save(temp_file)
            # Check that file was created
            self.assertTrue(os.path.exists(temp_file))
        finally:
            # Clean up
            if os.path.exists(temp_file):
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
        
        try:
            # Create a graph
            self.db.create_graph("test_graph")
            
            # Load data from file
            self.db.load(temp_file)
            # If no exception is raised, the test passes
        finally:
            # Clean up
            if os.path.exists(temp_file):
                os.unlink(temp_file)
    
    def test_error_handling(self):
        """Test error handling."""
        # Test that we can catch MiniGUError
        with self.assertRaises(MiniGUError):
            # Try to execute a query without creating a graph first
            # This might raise an error depending on implementation
            pass


class TestAsyncMiniGUAPI(unittest.IsolatedAsyncioTestCase):
    """Test cases for the async miniGU Python API."""
    
    async def asyncSetUp(self):
        """Set up test fixtures before each test method."""
        self.db = await async_connect()
    
    async def asyncTearDown(self):
        """Tear down test fixtures after each test method."""
        if self.db and self.db.is_connected:
            await self.db.close()
    
    async def test_async_connection(self):
        """Test basic asynchronous database connection."""
        self.assertIsInstance(self.db, AsyncMiniGU)
        self.assertTrue(self.db.is_connected)
    
    async def test_async_create_graph(self):
        """Test asynchronous graph creation."""
        await self.db.create_graph("test_graph")
        # If no exception is raised, the test passes
    
    async def test_async_insert_and_query_data(self):
        """Test asynchronous inserting and querying data."""
        # Create a graph
        await self.db.create_graph("test_graph")
        
        # Insert data
        sample_data = [
            {"name": "Alice", "age": 30, "label": "Person"},
            {"name": "Bob", "age": 25, "label": "Person"}
        ]
        await self.db.insert(sample_data)
        
        # Query data
        result = await self.db.execute("MATCH (n:Person) RETURN n.name, n.age")
        self.assertIsInstance(result, QueryResult)
        self.assertEqual(result.row_count, 2)
    
    async def test_async_performance_stats(self):
        """Test getting performance stats asynchronously."""
        stats = await self.db.get_performance_stats()
        self.assertIsInstance(stats, dict)
        # Check that expected keys are present
        self.assertIn("cache_hits", stats)
        self.assertIn("cache_misses", stats)
        self.assertIn("query_count", stats)


if __name__ == "__main__":
    unittest.main()