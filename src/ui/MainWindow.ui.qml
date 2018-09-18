import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.11

ApplicationWindow {
    visible: true
    minimumWidth:  1024
    minimumHeight: 800
    title: "Blitz Money"

    menuBar: MenuBar {

    }

    toolBar: ToolBar {
        RowLayout {
            anchors.fill: parent

            Item {
                Layout.fillWidth: true
            }

            ToolButton {
                action: manageAccountsAction
            }

        }
    }

    ManageAccountsWindow {
        id: manageAccountsWindow
    }

    Action {
        id: manageAccountsAction
        iconName: "address-book-new"
        text: qsTr("Manage Accounts")
        onTriggered: manageAccountsWindow.show()
    }
}
