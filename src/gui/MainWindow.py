# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from PyQt6.QtWidgets import QLabel, QMainWindow, QPushButton


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()

        self.setWindowTitle("Coin Tracker")

        coin_list = QLabel("Coins will eventually be displayed here")

        self.setCentralWidget(coin_list)
        self.show()
