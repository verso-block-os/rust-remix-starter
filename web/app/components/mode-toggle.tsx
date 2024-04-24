import { Theme, useTheme } from "remix-themes";

import { Moon, Sun } from "lucide-react";
import { Button } from "./ui/button";

export function ModeToggle() {
  const [theme, setTheme] = useTheme();
  return (
    <Button
      size="icon"
      variant="secondary"
      onClick={() => {
        setTheme(theme === Theme.LIGHT ? Theme.DARK : Theme.LIGHT);
      }}
    >
      {theme === Theme.LIGHT ? (
        <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      ) : (
        <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
      )}
    </Button>
  );
}
