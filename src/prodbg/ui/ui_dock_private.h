#pragma once

#include "api/plugin_instance.h"
#include <vector> // TODO: replace with custom arrays

const int g_sizerSize = 4; // TODO: Move to settings

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum UIDockSizerDir
{
	UIDockSizerDir_Horz,
	UIDockSizerDir_Vert,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum UIDockType
{
	UIDockType_Docked,
	UIDockType_Floating,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum UIDockSide
{
	UIDockSide_Top,
	UIDockSide_Bottom,
	UIDockSide_Right,
	UIDockSide_Left,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct UIDock;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct UIDockSizer
{
	std::vector<UIDock*> side0;	// docks connected to up/left 
	std::vector<UIDock*> side1;	// docks connected to down/right 
	UIDockSizerDir dir;
	Rect rect; 
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct UIDock
{
	inline UIDock(ViewPluginInstance* inView) : 
		topSizer(0), bottomSizer(0), rightSizer(0), leftSizer(0), view(inView), type(UIDockType_Docked) 
	{ 
	}

	UIDockSizer* topSizer;
	UIDockSizer* bottomSizer;
	UIDockSizer* rightSizer;
	UIDockSizer* leftSizer;
	ViewPluginInstance* view;

	UIDockType type;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

enum UIDockStatus
{
	UIDockStatus_ok,
	UIDockStatus_fail,
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

struct UIDockingGrid
{
	std::vector<UIDock*> docks;
	std::vector<UIDockSizer*> sizers;
	UIDockSizer topSizer;
	UIDockSizer bottomSizer;
	UIDockSizer rightSizer;
	UIDockSizer leftSizer;
	Rect rect;
};


