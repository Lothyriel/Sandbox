#pragma once
#include <fstream>
#include <iostream>

class Logger
{
public:
	static void log(std::string message)
	{
		std::ofstream outdata;

		outdata.open("C:\\Users\\JX\\Downloads\\log.txt", std::ios_base::app);

		outdata << message << std::endl;

		outdata.close();
	}
};

