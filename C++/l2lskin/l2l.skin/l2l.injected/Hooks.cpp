#include <string>

#include "fnv_hash.hpp"

#include "CheatManager.hpp"
#include "Hooks.hpp"
#include "Memory.hpp"
#include "SDK/AIBaseCommon.hpp"
#include "SDK/GameState.hpp"
#include "Utils.hpp"
#include "vmt_smart_hook.hpp"
#include "Logger.hpp"

void randomizeSkins()
{
	const auto heroes{ cheatManager.memory->heroList };
	Logger::log("Detectados " + std::to_string(cheatManager.memory->heroList->length) + " campeões");

	for (auto i{ 0u }; i < heroes->length; ++i) {
		const auto hero{ heroes->list[i] };
		const auto championHash{ fnv::hash_runtime(hero->get_character_data_stack()->base_skin.model.str) };

		if (championHash == FNV("PracticeTool_TargetDummy"))
			continue;

		Logger::log("Trocando skin do: " + *hero->get_name());
		const auto skinCount{ cheatManager.database->champions_skins[championHash].size() };

		Logger::log("Encontradas " + std::to_string(skinCount) + " skins");
		auto& skinDatabase{ cheatManager.database->champions_skins[championHash] };
		auto aleatorio = random(1u, skinCount);

		Logger::log("Numero aleatorio sacado: " + std::to_string(aleatorio));

		auto& skin = skinDatabase[aleatorio - 1];

		Logger::log("Skin escolhida: " + skin.skin_name);

		hero->change_skin(skin.model_name, skin.skin_id);
	}
}

static void changeModelForObject(const AIBaseCommon* obj, const char* model, const std::int32_t skin) noexcept
{
	if (skin == -1)
		return;

	if (const auto stack{ obj->get_character_data_stack() }; stack->base_skin.skin != skin) {
		stack->base_skin.skin = skin;
		stack->stack.clear();
		stack->push(model, skin);
	}
}

static void changeSkinForObject(const AIBaseCommon* obj, const std::int32_t skin) noexcept
{
	if (skin == -1)
		return;

	if (const auto stack{ obj->get_character_data_stack() }; stack->base_skin.skin != skin) {
		stack->base_skin.skin = skin;
		stack->update(true);
	}
}

void Hooks::run() noexcept
{
	Logger::log("Hooks ligados");

	const auto client{ cheatManager.memory->client };
	if (client) {
		if (client->game_state == GGameState_s::Running) {
			randomizeSkins();
			Logger::log("Hooks Inicializados");
		}
		else if (client->game_state == GGameState_s::Connecting) {
			Logger::log("Game State: Connecting");
		}
		else if (client->game_state == GGameState_s::LoadingScreen) {
			Logger::log("Game State: LoadingScreen");
		}
		else if (client->game_state == GGameState_s::Paused) {
			Logger::log("Game State: Paused");
		}
		else if (client->game_state == GGameState_s::Finished) {
			Logger::log("Game State: Finished");
		}
		else if (client->game_state == GGameState_s::Exiting) {
			Logger::log("Game State: Exiting");
		}
	}
}

void Hooks::init()
{
	const auto player{ cheatManager.memory->localPlayer };
	const auto heroes{ cheatManager.memory->heroList };
	const auto minions{ cheatManager.memory->minionList };
	static const auto playerHash{ player ? fnv::hash_runtime(player->get_character_data_stack()->base_skin.model.str) : 0u };

	for (auto i{ 0u }; i < heroes->length; ++i) {
		if (const auto hero{ heroes->list[i] }; hero->get_character_data_stack()->stack.size() > 0) {
			// Viego transforms into another champion as 2nd form, our own skin's id may not match for every champion. (same problem exists in sylas) 
			if (const auto championName{ fnv::hash_runtime(hero->get_character_data_stack()->base_skin.model.str) }; championName == FNV("Viego") || championName == FNV("Sylas"))
				continue;

			if (auto& stack{ hero->get_character_data_stack()->stack.front() }; stack.skin != hero->get_character_data_stack()->base_skin.skin) {
				stack.skin = hero->get_character_data_stack()->base_skin.skin;
				hero->get_character_data_stack()->update(true);
			}
		}
	}

	for (auto i{ 0u }; i < minions->length; ++i) {
		const auto minion{ minions->list[i] };

		const auto hash{ fnv::hash_runtime(minion->get_character_data_stack()->base_skin.model.str) };

		if (const auto owner{ minion->getGoldRedirectTarget() }; owner) {
			if (hash == FNV("JammerDevice") || hash == FNV("SightWard") || hash == FNV("YellowTrinket") || hash == FNV("VisionWard") || hash == FNV("BlueTrinket") || hash == FNV("TestCubeRender10Vision")) {
				if (!player || owner == player) {
					if (hash == FNV("TestCubeRender10Vision") && playerHash == FNV("Yone"))
						changeModelForObject(minion, "Yone", owner->get_character_data_stack()->base_skin.skin);
					else if (hash == FNV("TestCubeRender10Vision"))
						changeSkinForObject(minion, 0);
				}
			}
			else if (hash != FNV("SRU_Jungle_Companions") && hash != FNV("DominationScout"))
				changeSkinForObject(minion, owner->get_character_data_stack()->base_skin.skin);
			continue;
		}

		// Just LocalPlayer
		if ((hash == FNV("NunuSnowball") && playerHash == FNV("Nunu")) || (hash == FNV("KindredWolf") && playerHash == FNV("Kindred")) || (hash == FNV("QuinnValor") && playerHash == FNV("Quinn")))
			changeSkinForObject(minion, player->get_character_data_stack()->base_skin.skin);
	}
}
