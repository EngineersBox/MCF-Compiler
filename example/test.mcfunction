#alias entity playerTalkedToAll @a[scores={candice=1,dang=1,peter=1,ben=1,alicia=1,lynne=1,cherida=1,james=1,jess=1,karen=1,aabira=1,bron=1,lauren=1,gareth=1,murphy=1,josh=1,bucket=1}]

# Unblock barrier to return to the first area
execute @a[scores={candice=1,dang=1,peter=1,ben=1,alicia=1,lynne=1,cherida=1,james=1,jess=1,karen=1,aabira=1,bron=1,lauren=1,gareth=1,murphy=1,josh=1,bucket=1}] ~ ~ ~ fill 767 46 35 771 44 35 air
execute @a[scores={candice=1,dang=1,peter=1,ben=1,alicia=1,lynne=1,cherida=1,james=1,jess=1,karen=1,aabira=1,bron=1,lauren=1,gareth=1,murphy=1,josh=1,bucket=1}] ~ ~ ~ tp @e[type=npc,tag=moveUp] ~ ~4 ~
execute @a[scores={candice=1,dang=1,peter=1,ben=1,alicia=1,lynne=1,cherida=1,james=1,jess=1,karen=1,aabira=1,bron=1,lauren=1,gareth=1,murphy=1,josh=1,bucket=1}] ~ ~ ~ scoreboard objectives remove npcTalkedTo
