# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from gui.MainWindow import MainWindow
from PyQt6.QtWidgets import QApplication
import sys


EXECUTABLE_NAME = "coin-tracker"
VERSION = "0.0.1"


def main():

    print("start of if")

    if len(sys.argv) >= 2 and sys.argv[1] in ("--version", "-v"):
        print(EXECUTABLE_NAME + " " + VERSION)
        quit()

    print("after if")
    app = QApplication(sys.argv)
    app.setApplicationVersion(VERSION)

    mainWindow = MainWindow()
    mainWindow.show()

    app.exec()


if __name__ == "__main__":
    main()
