# Main module initialization file
try:
    from .minigu_python import PyMiniGU
    __all__ = ['PyMiniGU']
except ImportError:
    # Fallback if the Rust extension is not available
    pass