# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

import argparse
from database import Database
from gui.MainWindow import MainWindow
from PyQt6.QtWidgets import QApplication
import sys


EXECUTABLE_NAME = "coin-tracker"
VERSION = "0.0.0"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-v",
        "--version",
        help="Show version information",
        action="store_true"
    )
    args = parser.parse_args()
    if args.version:
        print(f"{EXECUTABLE_NAME}: {VERSION}")
        quit(0)

    db = Database()

    app = QApplication(sys.argv)
    app.setApplicationVersion(VERSION)

    mainWindow = MainWindow(db)
    mainWindow.show()

    app.exec()


if __name__ == "__main__":
    main()
