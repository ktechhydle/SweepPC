from src.imports import *


class SweepPC(QtWidgets.QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle('SweepPC')
        self.setWindowIcon(QtGui.QIcon('resources/icons/sweep_pc_icon.svg'))
        self.resize(1000, 800)

        self.createUI()

    def createUI(self):
        pass


def main():
    app = QtWidgets.QApplication([])

    window = SweepPC()
    window.show()

    app.exec()

if __name__ == "__main__":
    main()