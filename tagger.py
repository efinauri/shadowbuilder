import json
import os


# TODO: tags for ul

def tagger(craft, ul: bool):
    """
    :param craft: full and capitalized name, e.g. Portalcraft.
    :param ul: True/False for Rotation/Unlimited.
    """
    filename = f'{os.getcwd()}/shadowverse-json/{craft}.json'
    with open(filename, 'r') as f:
        data = json.load(f)
    data_tokens = {}  # This temporary card library is used for tagging cards according to what tokens they make.
    # Filters the card library by the class and game mode specified above.
    data_cards = {}
    for i in list(data):
        if data[i]["expansion_"] == "Token":
            data_tokens[i] = data[i]
        elif not (not ul and not data[i]["rotation_"]):
            data_cards[i] = data[i]
    del data
    for i in data_cards:
        card = data_cards[i]
        effect = card["baseEffect_"] + ' ' + card["evoEffect_"]
        tokens = [tk for tk in list(data_tokens) if tk[:-1] in effect]

        for tk in tokens:
            effect += ' ' + data_tokens[tk]["baseEffect_"] + ' ' + data_tokens[tk]["evoEffect_"]
        try:
            tags = card["tags_"]
        except KeyError:
            tags = []
        # A list of automatic tags that come to mind follows:
        #################
        # CRAFT GENERIC # evo, mc, nat
        #################
        if True in (term in effect for term in ("followers evolved", "have evolved", "volve this", "volve all",
                                                "for 0 evolution points", "volve it", "Union Burst")):
            tags.append("evo")
        if "Mach" in card["trait_"] or "achina" in effect:
            tags.append("mc")
        if "Great Tree" in effect or "atura " in effect or "Nat" in card["trait_"]:
            tags.append("nat")
        ##########
        # FOREST # amazon, fairy, accel
        ##########
        if craft == "Forestcraft" or craft == "Neutral":
            if "Greenwood" in effect:
                tags.append("amazon")
            if "Fair" in effect:
                tags.append("fairy")
            if "ccelerate" in effect:
                tags.append("accel")
        #########
        # SWORD # levin, rally
        #########
        elif craft == "Swordcraft" or craft == "Neutral":
            if card["trait_"] == "Levin":
                tags.append("levin")
            if "ummon" in effect or "Rally" in effect:
                tags.append("rally")
        ########
        # RUNE # dirt, sboost
        ########
        elif craft == "Runecraft" or craft == "Neutral":
            if card["trait_"] == "Earth Sigil" \
                    or True in (i in effect for i in ("arth essence", "cauldron", "Earth Rite", "amulet")) \
                    and "this amulet" not in effect:  # Item Shop, Sagacious Core
                tags.append("dirt")
            if "pellboost" in effect:
                tags.append("sboost")
        ##########
        # DRAGON # disco, ramp
        ##########
        elif craft == "Dragoncraft" or craft == "Neutral":
            if "iscard" in effect:
                tags.append("disco")
            if "empty play point" in effect or "Overflow" in effect:
                tags.append("ramp")
        ##########
        # SHADOW # necro, burial, lw
        ##########
        elif craft == "Shadowcraft" or craft == "Neutral":
            if "Necromancy" in effect or "Shadows" in effect:
                tags.append("necro")
            if "Burial Rite" in effect or "Reanimate" in effect or "hen destroy" in effect:
                tags.append("burial")
            if "Last Words" in effect:
                tags.append("lw")
        #########
        # BLOOD # avarice, wrath, veng
        #########
        elif craft == "Bloodcraft" or craft == "Neutral":
            if "Avarice" in effect or "draw" in effect.lower():
                tags.append("avarice")
            if True in (term in effect for term in ("Wrath", "damage to your leader", "damage to both leaders",
                                                    "leader takes damage", "has taken damage")):
                tags.append("wrath")
            if "Vengeance" in effect:
                tags.append("veng")
        #########
        # HAVEN # heal, amulet, ward
        #########
        elif craft == "Havencraft" or craft == "Neutral":
            if "restore" in effect or "elana" in effect.lower() or "repair mode" in effect:
                tags.append("heal")
            if card["type_"] == "Amulet" or "countdown" in effect or "Crystallize" in effect or "mulet" in effect:
                tags.append("amulet")
            if "Ward" in effect in effect:
                tags.append("ward")
        ##########
        # PORTAL # af, puppet, float
        ##########
        elif craft == "Portalcraft" or craft == "Neutral":
            if "rtifact" in effect:
                tags.append("af")
            if "uppet" in effect:
                tags.append("puppet")
            if False not in (term in effect for term in ("end of your turn", "at least", "play point")):
                tags.append("float")
        card["tags_"] = tags

    with open(f'{os.getcwd()}/src/assets/{craft}_{"Unlimited" if ul else "Rotation"}.json', 'w+') as f:
        json.dump(data_cards, f)


#  Generates all the json files.
for i in ["Neutral", "Forest", "Sword", "Rune",
          "Dragon", "Shadow", "Blood", "Haven", "Portal"]:
    tagger(i + "craft" * (i[0] != 'N'), ul=True)
    tagger(i + "craft" * (i[0] != 'N'), ul=False)
