/**
 * Blitz Money
 *
 * Copyright 2018 Luis Fernando Batels <luisfbatels@gmail.com>
 */
#include <QApplication>
#include <QMessageBox>
#include <QMainWindow>
#include <QWindow>
#include <QToolBar>

#include "qtbindings.h"
#include "gui.h"

/**
 * Main window of the application
 */
int main(int argc, char* argv[]) {

    QApplication app(argc, argv);

    QMainWindow* main = new QMainWindow();
    main->setMinimumSize(QSize(1024, 800));


    // ToolBar
    QToolBar* toolbar = new QToolBar();
    toolbar->addAction(QIcon::fromTheme("address-book-new"), "Gerenciar contas", manage_accounts);


    main->addToolBar(toolbar);
    main->show();

    return app.exec();
}

void manage_accounts() {

    QWindow* window = new QWindow();
    window->setMinimumSize(QSize(500, 500));
    window->setModality(Qt::ApplicationModal);

    window->show();

    //QCoreApplication::quit();
}
