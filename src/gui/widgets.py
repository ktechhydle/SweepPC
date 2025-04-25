from src.imports import *


class RunButton(QPushButton):
    def __init__(self, text, parent=None):
        super().__init__(text, parent)
        self.setFixedSize(80, 80)
        self.setObjectName('#runButton')