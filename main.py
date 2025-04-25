from src.imports import *
from src.gui.panels import MainPanel, HomePanel
from mp_software_stylesheets.styles import SWEEPPCCSS


class SweepPC(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('SweepPC')
        self.setWindowIcon(QIcon('resources/icons/sweep_pc_icon.svg'))
        self.resize(1000, 800)

        self.createUI()

    def createUI(self):
        self.main_panel = MainPanel(self)
        self.setCentralWidget(self.main_panel)

        self.home_panel = HomePanel(self)
        self.main_panel.addPage('Home', self.home_panel)
        self.main_panel.finish()


def main():
    app = QApplication([])
    app.setStyleSheet(SWEEPPCCSS)

    window = SweepPC()
    window.show()

    app.exec()

if __name__ == "__main__":
    main()