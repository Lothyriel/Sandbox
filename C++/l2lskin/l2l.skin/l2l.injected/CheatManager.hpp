#pragma once

#include <memory>

#include "Hooks.hpp"
#include "Memory.hpp"
#include "SkinDatabase.hpp"

class CheatManager {
public:
	void start() noexcept
	{
		this->hooks = std::make_unique<Hooks>();
		this->memory = std::make_unique<Memory>();
		this->database = std::make_unique<SkinDatabase>();
	}

	bool cheatState{ true };
	std::unique_ptr<Hooks> hooks;
	std::unique_ptr<Memory> memory;
	std::unique_ptr<SkinDatabase> database;
};

inline CheatManager cheatManager;
