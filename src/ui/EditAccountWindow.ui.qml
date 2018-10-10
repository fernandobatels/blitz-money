import QtQuick 2.6
import QtQuick.Window 2.2
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.0
import org.kde.plasma.components 3 as PlasmaComponents

ApplicationWindow {
    minimumWidth:  450
    minimumHeight: 230
    modality: Qt.ApplicationModal
    title: qsTr("Manage Account")

    GridLayout {
        id: gridLayout

        anchors.right: parent.right
        anchors.left: parent.left
        anchors.top: parent.top
        anchors.rightMargin: 12
        anchors.leftMargin: 12
        anchors.topMargin: 12
        columnSpacing: 8
        rowSpacing: 8
        rows: 5
        columns: 4

        PlasmaComponents.Label {
            text: qsTr("Name")
            Layout.columnSpan: 2
        }

        PlasmaComponents.TextField {
            id: name
            Layout.minimumWidth: 140
            Layout.fillWidth: true
            Layout.columnSpan: 3
            placeholderText: qsTr("Name of account")
        }

        PlasmaComponents.Label {
            text: qsTr("Bank")
            Layout.columnSpan: 2
        }

        PlasmaComponents.TextField {
            id: bank
            Layout.minimumWidth: 140
            Layout.fillWidth: true
            Layout.columnSpan: 4
            placeholderText: qsTr("Bank name")
        }

        PlasmaComponents.Label {
            text: qsTr("Opening Balance")
            Layout.columnSpan: 2
        }

        PlasmaComponents.Label {
            text: qsTr("Opening Balance Date")
            Layout.columnSpan: 2
        }

        PlasmaComponents.TextField {
            id: openingbalance
            inputMethodHints: Qt.ImhFormattedNumbersOnly
            Layout.minimumWidth: 140
            Layout.fillWidth: true
            Layout.columnSpan: 1
        }

        Item {
            Layout.preferredHeight: 14
            Layout.preferredWidth: 14
        }

        PlasmaComponents.TextField {
            id: openingbalancedate
            inputMethodHints: Qt.ImhDate
            Layout.minimumWidth: 140
            Layout.fillWidth: true
            Layout.columnSpan: 1
        }
    }

    statusBar: RowLayout {
        anchors.topMargin: 12
        anchors.right: parent.right
        anchors.rightMargin: 12

        PlasmaComponents.Button {
            id: save
            text: qsTr("Save")
            //icon: "document-save"
        }

        PlasmaComponents.Button {
            id: cancel
            text: qsTr("Cancel")
            //icon: "document-revert"
            onClicked: editAccountWindow.close()
        }

    }
}
