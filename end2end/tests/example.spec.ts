import { test, expect } from "@playwright/test";

test("person page has title and expected texts", async ({ page }) => {
  await page.goto("http://localhost:3000/person/stephan");

  await expect(page).toHaveTitle("Welcome to Leptos");
  await expect(page.locator("h1")).toHaveText("Hello stephan, welcome to Leptos!");
  await expect(page.locator("span")).toHaveText("Bevy ECS + Leptos = 💕");
});
