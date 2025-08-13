from ..python.minigu import connect, MiniGU, QueryResult, MiniGUError

__all__ = ['connect', 'MiniGU', 'QueryResult', 'MiniGUError']
from .minigu import connect, MiniGU, QueryResult, MiniGUError

# 确保导出所有公共接口
__all__ = ['connect', 'MiniGU', 'QueryResult', 'MiniGUError']