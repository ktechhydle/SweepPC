import sweep_pc
from src.imports import *


class BasePanel(QtWidgets.QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)


class HomePanel(BasePanel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setLayout(QtWidgets.QVBoxLayout())

    def startCleanup(self):
        pass