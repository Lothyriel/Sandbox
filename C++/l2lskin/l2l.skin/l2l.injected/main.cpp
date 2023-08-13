#pragma warning(disable : 6387 4715)

#include <Windows.h>
#include <array>
#include <clocale>
#include <chrono>
#include <thread>

#include "CheatManager.hpp"

#include "Hooks.hpp"
#include "Memory.hpp"
#include "SDK/GameState.hpp"
#include "Logger.hpp"


bool WINAPI HideThread(const HANDLE hThread) noexcept
{
	__try {
		using FnSetInformationThread = NTSTATUS(NTAPI*)(HANDLE ThreadHandle, UINT ThreadInformationClass, PVOID ThreadInformation, ULONG ThreadInformationLength);
		const auto NtSetInformationThread{ reinterpret_cast<FnSetInformationThread>(::GetProcAddress(::GetModuleHandle(L"ntdll.dll"), "NtSetInformationThread")) };

		if (!NtSetInformationThread)
			return false;

		if (const auto status{ NtSetInformationThread(hThread, 0x11u, nullptr, 0ul) }; status == 0x00000000)
			return true;
	}
	__except (TRUE) {
		return false;
	}
}

__declspec(safebuffers) static void WINAPI DllAttach([[maybe_unused]] LPVOID lp) noexcept
{
	using namespace std::chrono_literals;

	Logger::log("Dll atochada, iniciando cheat");

	cheatManager.start();

	Logger::log("Cheat iniciado");

	HideThread(::GetCurrentThread());

	cheatManager.memory->Search(true);
	while (true) {
		std::this_thread::sleep_for(1s);

		if (!cheatManager.memory->client)
			cheatManager.memory->Search(true);
		else if (cheatManager.memory->client->game_state == GGameState_s::Running)
			break;
	}

	Logger::log("Jogo encontrado");

	const auto gadget{ *reinterpret_cast<std::array<std::uint8_t, 2>*>(cheatManager.memory->base + offsets::global::retSpoofGadget) };

	invoker.init(cheatManager.memory->base + offsets::global::retSpoofGadget);

	std::this_thread::sleep_for(500ms);
	cheatManager.memory->Search(false);
	std::this_thread::sleep_for(500ms);
	Logger::log("Memória inicializada");

	cheatManager.database->load();

	cheatManager.hooks->init();
	cheatManager.hooks->run();

	while (cheatManager.cheatState) {
		std::this_thread::sleep_for(250ms);
	}

	Logger::log("Saindo do processo");
	::ExitProcess(0u);
}

__declspec(safebuffers)BOOL APIENTRY DllMain(const HMODULE hModule, const DWORD reason, [[maybe_unused]] LPVOID reserved)
{
	DisableThreadLibraryCalls(hModule);

	if (reason != DLL_PROCESS_ATTACH)
		return FALSE;

	HideThread(hModule);
	std::setlocale(LC_ALL, ".utf8");

	::_beginthreadex(nullptr, 0u, reinterpret_cast<_beginthreadex_proc_type>(DllAttach), nullptr, 0u, nullptr);
	::CloseHandle(hModule);
	return TRUE;
}
