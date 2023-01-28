#!/usr/bin/python3
from Game_map import GameMap
import pygame
import sys
import os

def main():
    chance = GameMap("examples/map.json").chance
    for key in chance.res:
        print(f"{key}, {chance.item_chance(key) * 100:.2f}%")
    # m
    clock = pygame.time.Clock()
    screen = pygame.display.set_mode((600, 480))
    # Load the background image here. Make sure the file exists!
    bg = pygame.image.load(os.path.join("./assets", "none.png"))
    pygame.mouse.set_visible(0)
    # fix indentation
    while True:
        clock.tick(60)
        screen.blit(bg, (0, 0))
        x, y = pygame.mouse.get_pos()
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                sys.exit()
        pygame.display.update()

if __name__ == "__main__" :
    pygame.init()
    pygame.display.set_caption('The Settlers of Python')
    main()
    

