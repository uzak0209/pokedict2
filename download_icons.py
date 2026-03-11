import os
import urllib.request
import urllib.error
import time
from concurrent.futures import ThreadPoolExecutor

# Directory to save icons
SAVE_DIR = "frontend/public/icons/pokemon"
os.makedirs(SAVE_DIR, exist_ok=True)

# Base URL for PokeAPI sprites (pixel art)
BASE_URL = "https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png"

def download_icon(pokemon_id):
    url = BASE_URL.format(pokemon_id)
    save_path = os.path.join(SAVE_DIR, f"{pokemon_id}.png")
    
    if os.path.exists(save_path):
        return # Skip if already exists

    try:
        with urllib.request.urlopen(url, timeout=10) as response:
            if response.status == 200:
                with open(save_path, 'wb') as f:
                    f.write(response.read())
                print(f"Downloaded: {pokemon_id}")
    except urllib.error.HTTPError as e:
        print(f"Failed (HTTP {e.code}): {pokemon_id}")
    except Exception as e:
        print(f"Error downloading {pokemon_id}: {e}")

def main():
    # Standard National Dex
    ids = list(range(1, 1026))
    
    # Special forms
    ids.extend(range(10001, 10278))

    print(f"Starting download for {len(ids)} icons...")
    
    with ThreadPoolExecutor(max_workers=20) as executor:
        executor.map(download_icon, ids)

    print("Download complete.")

if __name__ == "__main__":
    main()
