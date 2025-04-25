import sweep_pc
from src.imports import *


class BasePanel(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)


class MainPanel(BasePanel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setLayout(QHBoxLayout())

        self._current_widget = None
        self._button_group = QButtonGroup(self)

        self.createUI()

    def createUI(self):
        self.navigator_panel = QWidget(self)
        self.navigator_panel.setFixedWidth(200)
        self.navigator_panel.setLayout(QVBoxLayout())
        self.layout().addWidget(self.navigator_panel)

        self.stack = QStackedWidget(self)
        self.layout().addWidget(self.stack)

    def addPage(self, text: str, widget: QWidget):
        button = QPushButton(text)
        button.setCheckable(True)
        self._button_group.addButton(button)
        self.navigator_panel.layout().addWidget(button)

        self.stack.addWidget(widget)

        index = self.stack.indexOf(widget)
        button.clicked.connect(lambda _, idx=index: self.setCurrentPage(idx))

        # select the first page
        if self.stack.count() == 1:
            button.setChecked(True)
            self.stack.setCurrentIndex(0)

    def setCurrentPage(self, index: int):
        self.stack.setCurrentIndex(index)


class HomePanel(BasePanel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setLayout(QVBoxLayout())

        self.createUI()

    def createUI(self):
        label = QLabel('Lets get started with a quick cleanup of your PC.')
        label.setAlignment(Qt.AlignmentFlag.AlignCenter)

        run_btn = QPushButton('Run')
        run_btn.setObjectName('#runButton')
        run_btn.clicked.connect(self.startCleanup)

        self.layout().addStretch()
        self.layout().addWidget(label)
        self.layout().addWidget(run_btn)

    def startCleanup(self):
        pass