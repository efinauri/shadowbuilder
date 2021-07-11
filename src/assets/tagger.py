import json


def load_cardpool(rot: bool):
    with open('all.json', 'r') as f:
        raw_all = json.load(f)
        all = {}
        for k, info in raw_all.items():
            raw_all[k]["effect_"] = info["baseEffect_"] + " " + info["evoEffect_"] + " "
        for k, info in raw_all.items():
            if info["expansion_"] == "Token":
                continue
            for tk in info["tokens_"]:
                info["effect_"] += raw_all[str(tk)]["effect_"] + " " + raw_all[str(tk)]["trait_"]
            info["effect_"] = info["effect_"].lower()
            del info["rarity_"]
            del info["expansion_"]
            del info["alts_"]
            del info["tokens_"]
            del info["baseEffect_"]
            del info["baseFlair_"]
            del info["evoEffect_"]
            del info["evoFlair_"]
            all[k] = info
    with open('_tmp.json', 'w+') as f:
        json.dump(all, f, ensure_ascii=False)
    return all


def autotag_card(card):
    eff = card["effect_"]
    craft = card["craft_"]
    tags = set()
    #################
    # CRAFT GENERIC # evo, mc, nat
    #################
    if any(i in eff for i in ("evolve", "evolution", "union burst")):
        tags.add("Evolve")
    if "Mach" in card["trait_"] or "machina" in eff:
        tags.add("Machina")
    if "naterra" in eff or "natura" in eff or "Nat" in card["trait_"]:
        tags.add("Natura")
    ##########
    # FOREST # amazon, fairy, accel
    ##########
    if craft == "Forestcraft" or craft == "Neutral":
        if "greenwood" in eff:
            tags.add("Amazon")
        if "fairy" in eff or "fairies" in eff:
            tags.add("Fairy")
        if "accelerate" in eff:
            tags.add("Accelerate")
        if '+' in eff or "increased by an effect":
            tags.add("Buff")
    #########
    # SWORD # levin, rally
    #########
    elif craft == "Swordcraft" or craft == "Neutral":
        if card["trait_"] == "Levin" or "Lvn" in card["trait_"] or "levin" in eff:
            tags.add("Levin")
        if "summon" in eff or "rally" in eff:
            tags.add("Rally")
    ########
    # RUNE # dirt, sboost
    ########
    elif craft == "Runecraft" or craft == "Neutral":
        if card["trait_"] == "Earth Sigil" \
                or any(i in eff for i in ("earth sigil", "earth rite")):  # Item Shop, Sagacious Core
            tags.add("Dirt")
        if "spellboost" in eff or card["type_"] == "Spell":
            tags.add("Spellboost")
    ##########
    # DRAGON # disco, ramp
    ##########
    elif craft == "Dragoncraft" or craft == "Neutral":
        if "discard" in eff:
            tags.add("Discard")
        if "empty play point" in eff or "overflow" in eff:
            tags.add("Ramp")
    ##########
    # SHADOW # necro, burial, lw
    ##########
    elif craft == "Shadowcraft" or craft == "Neutral":
        if "necromancy" in eff or ("gain" in eff and "shadow" in eff):
            tags.add("Necromancy")
        if "burial rite" in eff or "reanimate" in eff or "then destroy" in eff:
            tags.add("Burial Rite")
        if "last words" in eff:
            tags.add("Last Words")
    #########
    # BLOOD # avarice, wrath, veng
    #########
    elif craft == "Bloodcraft" or craft == "Neutral":
        if "avarice" in eff or "draw" in eff:
            tags.add("Avarice")
        if any(i in eff for i in ("wrath", "damage to your leader", "damage to both leaders",
                                  "leader takes damage", "has taken damage")):
            tags.add("Wrath")
        if "vengeance" in eff:
            tags.add("Vengeance")
    #########
    # HAVEN # heal, amulet, ward
    #########
    elif craft == "Havencraft" or craft == "Neutral":
        if "restore" in eff or "elana" in eff or "repair mode" in eff:
            tags.add("Heal")
        if "countdown" in eff or "amulet" in eff:
            tags.add("Amulet")
        if "ward" in eff:
            tags.add("Ward")
    ##########
    # PORTAL # af, puppet, float
    ##########
    elif craft == "Portalcraft" or craft == "Neutral":
        if "artifact" in eff:
            tags.add("Artifact")
        if "puppet" in eff:
            tags.add("Puppet")
        if "play point" in eff:
            tags.add("Float")
    return tags


if __name__ == "__main__":
    pool = load_cardpool(False)
    for i, card in pool.items():
        card["tags_"] = list(autotag_card(card))
        pool[i] = card
    with open('cards.json', 'w+') as f:
        json.dump(pool, f, ensure_ascii=False)
