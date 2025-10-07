from coin import Coin, CoinType
from PyQt6.QtWidgets import (
    QLabel,
    QVBoxLayout,
    QWidget,
)
from typing import List


class CollectionEntry(QLabel):
    def __init__(self, coin: Coin):
        super().__init__()
        self.text = f"{coin.name}\n"

        if coin.country == coin.issuer:
            self.text += coin.country
        else:
            self.text += coin.country + " / " + coin.issuer

        self.text += " | " + str(coin.min_year) + " - " + str(coin.max_year) + "\n"
        self.text += str(CoinType(coin.coin_type))

        self.setText(self.text)


class CollectionView(QWidget):
    def __init__(self, collection: List[Coin] = None, entriesPerPage: int = 10):
        super().__init__()

        self.entriesPerPage = entriesPerPage

        layout = QVBoxLayout()
        self.setLayout(layout)

        # TODO: doesn't work, database func not returning None for empty list
        if collection is None:
            label = QLabel((
                "There are no coins to display.\n"
                "Try adding some to get started!"
            ))
            layout.addWidget(label)
            label.show()
            return

        self.entries = []
        for e in collection:
            entry = CollectionEntry(e)
            entry.setStyleSheet("""
                border: 1px solid #000000;
            """)
            layout.addWidget(entry)
            entry.show()
            self.entries.append(entry)
