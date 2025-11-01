#!/usr/bin/env python3.7
"""
Test cases for miniGU Python API.

This file contains tests for:
1. Basic connection functionality
2. Graph creation and management
3. Query execution
4. Result handling
5. Error handling
6. Async API functionality
7. Transaction methods
"""

import unittest
import asyncio
import sys
import os

# Add the python module to the path
sys.path.insert(0, os.path.join(os.path.dirname(__file__)))

import minigu


class TestMiniGUAPI(unittest.TestCase):
    def setUp(self):
        """Set up test fixtures before each test method."""
        self.db = minigu.MiniGU()
        self.test_graph_name = "test_graph_for_unit_tests"
        # Ensure connection for tests that require it
        if not self.db.is_connected:
            self.db._connect()

    def tearDown(self):
        """Tear down test fixtures after each test method."""
        pass

    def test_connect(self):
        """Test connecting to the database."""
        # Connection should be established in setUp
        self.assertTrue(self.db.is_connected)
        self.assertIsNotNone(self.db._rust_instance)

    def test_create_graph(self):
        """Test creating a graph."""
        # This should work without throwing exceptions
        self.db.create_graph("test_graph")
        # If we reach here, the test passes

    def test_begin_transaction(self):
        """Test beginning a transaction."""
        self.db.create_graph("test_graph_for_transaction")
        # This should raise TransactionError as the feature is not yet implemented
        with self.assertRaises(minigu.TransactionError):
            self.db.begin_transaction()

    def test_commit_transaction(self):
        """Test committing a transaction."""
        self.db.create_graph("test_graph_for_commit")
        # This should raise TransactionError as the feature is not yet implemented
        with self.assertRaises(minigu.TransactionError):
            self.db.commit()

    def test_rollback_transaction(self):
        """Test rolling back a transaction."""
        self.db.create_graph("test_graph_for_rollback")
        # This should raise TransactionError as the feature is not yet implemented
        with self.assertRaises(minigu.TransactionError):
            self.db.rollback()

    def test_transaction_methods(self):
        """Test transaction methods existence and basic functionality."""
        # Check that transaction methods exist
        self.assertTrue(hasattr(self.db, 'begin_transaction'))
        self.assertTrue(hasattr(self.db, 'commit'))
        self.assertTrue(hasattr(self.db, 'rollback'))

        # Test that transaction methods raise TransactionError as they are not yet implemented
        self.db.create_graph("test_graph_for_methods")
        with self.assertRaises(minigu.TransactionError):
            self.db.begin_transaction()


class TestAsyncMiniGUAPI(unittest.TestCase):
    def setUp(self):
        """Set up test fixtures before each test method."""
        self.loop = asyncio.new_event_loop()
        asyncio.set_event_loop(self.loop)
        self.db = minigu.AsyncMiniGU()
        self.test_graph_name = "test_graph_for_async_unit_tests"
        # Ensure connection for tests that require it
        if not self.db.is_connected:
            self.db._connect()

    def tearDown(self):
        """Tear down test fixtures after each test method."""
        self.loop.close()

    def test_async_connect(self):
        """Test connecting to the database asynchronously."""
        async def _test():
            self.assertTrue(self.db.is_connected)
            self.assertIsNotNone(self.db._rust_instance)
        
        self.loop.run_until_complete(_test())

    def test_async_create_graph(self):
        """Test creating a graph asynchronously."""
        async def _test():
            await self.db.create_graph("test_async_graph")
            # If no exception is raised, the test passes
        
        self.loop.run_until_complete(_test())

    def test_async_begin_transaction(self):
        """Test beginning a transaction asynchronously."""
        async def _test():
            await self.db.create_graph("test_async_transaction_graph")
            # This should raise TransactionError as the feature is not yet implemented
            with self.assertRaises(minigu.TransactionError):
                await self.db.begin_transaction()
        
        self.loop.run_until_complete(_test())

    def test_async_commit_transaction(self):
        """Test committing a transaction asynchronously."""
        async def _test():
            await self.db.create_graph("test_async_commit_graph")
            # This should raise TransactionError as the feature is not yet implemented
            with self.assertRaises(minigu.TransactionError):
                await self.db.commit()
        
        self.loop.run_until_complete(_test())

    def test_async_rollback_transaction(self):
        """Test rolling back a transaction asynchronously."""
        async def _test():
            await self.db.create_graph("test_async_rollback_graph")
            # This should raise TransactionError as the feature is not yet implemented
            with self.assertRaises(minigu.TransactionError):
                await self.db.rollback()
        
        self.loop.run_until_complete(_test())


if __name__ == '__main__':
    unittest.main()