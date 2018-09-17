/**
 * Blitz Money
 *
 * Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
 */
#include <QtQml/qqml.h>
#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QObject>
#include <QList>
#include <QQmlContext>

#include "qtbindings.h"

/**
 * Main window of the application
 */
int main(int argc, char* argv[]) {

    QGuiApplication app(argc, argv);
//    qmlRegisterType<Simple>("RustCode", 1, 0, "Simple");

    QQmlApplicationEngine engine;
    engine.load(QUrl(QStringLiteral("qrc:/MainWindow.ui.qml")));

    if (engine.rootObjects().isEmpty())
        return -1;

    return app.exec();
}
