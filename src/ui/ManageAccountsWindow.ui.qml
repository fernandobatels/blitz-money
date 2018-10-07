import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.0

ApplicationWindow {
    minimumWidth:  800
    minimumHeight: 500
    modality: Qt.ApplicationModal
    title: qsTr("Manage Accounts")

    toolBar: ToolBar {
        RowLayout {
            anchors.fill: parent

            Item {
                Layout.fillWidth: true
            }
        }
    }

    TableView {
        anchors.fill: parent
        model: sourceAccounts

        TableViewColumn {
            role: "name"
            title: qsTr("Name")
            width: 200
       }

        TableViewColumn {
            role: "bank"
            title: qsTr("Bank")
            width: 150
       }

        TableViewColumn {
            role: "openingbalance"
            title: qsTr("Opening Balance")
            width: 100
       }

        TableViewColumn {
            role: "openingbalancedate"
            title: qsTr("Opening Balance Date")
            width: 100
       }

        TableViewColumn {
            role: "uuid"
            title: "#id"
            width: 50
        }
   }
}
