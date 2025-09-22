# this file is part of coin-tracker by opdavi1 and subject to the GNU GPL-3.0-or-later license.
# See LICENSE for details or go to <https://www.gnu.org/licenses/>

from database import Database
from gui import Gui

# from coin import Coin


def main():
    db = Database()
    Gui()
    db.close()


if __name__ == "__main__":
    main()
