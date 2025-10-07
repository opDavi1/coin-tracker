# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from PyQt6.QtWidgets import (
    QApplication,
    QMainWindow,
    QPushButton
)
import sys


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Coin Tracker")

        button = QPushButton("Show database")
        button.pressed.connect(lambda: print("Showing database"))

        exit_button = QPushButton("Exit app")
        exit_button.pressed.connect(self.close)

        self.setCentralWidget(exit_button)
        self.show()


app = QApplication(sys.argv)
app.setApplicationName("Coin Tracker")
app.setApplicationVersion("0.0.1")
w = MainWindow()
app.exec()
