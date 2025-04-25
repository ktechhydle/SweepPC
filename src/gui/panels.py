from src.imports import *
from src.gui.widgets import RunButton
import sweep_pc


class BasePanel(QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)


class MainPanel(BasePanel):
    def __init__(self, parent=None):
        super().__init__(parent)
        self.setLayout(QHBoxLayout())
        self.layout().setContentsMargins(0, 0, 0, 0)

        self._current_widget = None
        self._button_group = QButtonGroup(self)

        self.createUI()

    def createUI(self):
        self.splitter = QSplitter(self)

        self.navigator_panel = QWidget(self)
        self.navigator_panel.setLayout(QVBoxLayout())
        self.splitter.addWidget(self.navigator_panel)

        self.stack = QStackedWidget(self)
        self.splitter.addWidget(self.stack)

        self.layout().addWidget(self.splitter)

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

        run_btn = RunButton('Run')
        run_btn.clicked.connect(self.startCleanup)

        self.layout().addStretch()
        self.layout().addWidget(label)
        self.layout().addWidget(run_btn, alignment=Qt.AlignmentFlag.AlignCenter)

    def startCleanup(self):
        pass