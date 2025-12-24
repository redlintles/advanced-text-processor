#!/usr/bin/python3

from pathlib import Path
import os
import shutil

ROOT = Path("./src/tokens/transforms")

HEADER_LINES = [
    '#[cfg(feature="test_access")]\n',
    'pub mod test;\n',
    '\n'
]

def main():
    for dirpath, dirnames, filenames in os.walk(ROOT):
        # Só processa arquivos .rs diretamente nessa pasta/nível
        for filename in filenames:
            if not filename.endswith(".rs"):
                continue

            source_path = Path(dirpath) / filename
            token_name = source_path.stem  # rnw.rs -> rnw
            folder_path = (Path(dirpath) / token_name).resolve()
            mod_path = folder_path / "mod.rs"
            test_path = folder_path / "test.rs"

            # Cria pasta do módulo (não explode se já existir)
            folder_path.mkdir(parents=True, exist_ok=True)

            # Move o arquivo original para mod.rs (se já existir mod.rs, você decide o que fazer)
            # Aqui: se mod.rs já existe, não sobrescreve automaticamente.
            if mod_path.exists():
                print(f"[SKIP] Já existe: {mod_path}")
                continue

            shutil.move(str(source_path), str(mod_path))

            # Lê e reescreve de forma segura (texto + encoding + cursor correto)
            with mod_path.open("r+", encoding="utf-8", newline="\n") as f:
                file_lines = f.readlines()

                # Evita duplicar se rodar duas vezes
                if file_lines[:len(HEADER_LINES)] != HEADER_LINES:
                    f.seek(0)          # ESSENCIAL
                    f.truncate()
                    f.writelines(HEADER_LINES + file_lines)

            # Cria test.rs se não existir
            test_path.touch(exist_ok=True)

            print(folder_path)

if __name__ == "__main__":
    main()
