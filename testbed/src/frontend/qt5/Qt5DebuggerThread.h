#pragma once

#include <QObject>
#include <QTimer>

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct PDDebugPlugin;

namespace prodbg
{

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class Qt5DebuggerThread : public QObject
{
	Q_OBJECT

public:
	Qt5DebuggerThread(const char* executable);

public slots:
    void start();
	void update();
 
signals:
    void finished();

private:

	PDDebugPlugin* m_debuggerPlugin;
	const char* m_executable;
	void* m_pluginData;
	QTimer m_timer;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

}

