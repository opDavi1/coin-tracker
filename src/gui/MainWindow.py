# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from PyQt6.QtWidgets import (
    QHBoxLayout,  # shoutout hbox
    QMainWindow,
)

from .CollectionView import CollectionView


class MainWindow(QMainWindow):
    def __init__(self, db):
        super().__init__()

        self.setWindowTitle("Coin Tracker")
        layout = QHBoxLayout()

        coins = db.get_all_coins()
        collectionView = CollectionView(coins)

        self.setLayout(layout)

        self.setCentralWidget(collectionView)
        self.show()
