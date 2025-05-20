from selenium import webdriver
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.by import By
from selenium.common.exceptions import *
from unittest import TestCase
from sample_text import sample


URL = "http://192.168.1.75:8081/"

cService = webdriver.ChromeService(executable_path='./chromedriver.exe')
driver = webdriver.Chrome(service=cService)
driver.get("http://192.168.1.75:8081/")


def create_post(n, category):
    try:
        filmes_category = driver.find_element(By.LINK_TEXT, f"{category}")
        filmes_category.click()

    except NoSuchElementException:
        print("Elemento nao encontrado")

    print(f"Categoria selencionada: {category}")
    print(f"URL atual: f{driver.current_url}")

    TestCase.assertTrue(
        driver.current_url == (URL + category), f"Erro ao acessar categoria: {category}")

    for i in range(n):
        post_text_area = driver.find_element(By.TAG_NAME, "textarea")
        post_text_area.clear()
        post_text_area.send_keys(f"Post {i} {sample}")

        post_btn = driver.find_element(By.ID, "post-btn")
        post_btn.click()

        post_text_area.clear()

        print("Post adicionado com sucesso")


create_post(5, "Filmes")
