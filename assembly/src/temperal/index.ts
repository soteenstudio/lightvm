// temperal.ts

class Temperal {
  // Kita simpan pointernya aja, bukan string murni, biar gak bentrok
  private registry: Map<string, usize> = new Map<string, usize>();

  set(key: string, data: string): void {
    // String.UTF8.encode itu alokasi otomatis di heap yang aman
    let buf = String.UTF8.encode(data);
    
    // Kita simpan pointer ke buffer tersebut
    // AS bakal jagain memori ini lewat Garbage Collector-nya
    this.registry.set(key, changetype<usize>(buf));
  }

  get(key: string): string {
    if (!this.registry.has(key)) {
      throw new Error("Temperal: Key '" + key + "' gak ketemu!");
    }

    let ptr = this.registry.get(key);
    
    // Balikin lagi dari pointer ke string
    return String.UTF8.decode(changetype<ArrayBuffer>(ptr));
  }

  has(key: string): bool {
    return this.registry.has(key);
  }

  delete(key: string): void {
    this.registry.delete(key);
  }

  clear(): void {
    this.registry.clear();
  }
}

const t = new Temperal();

// Export tetap sama biar gak ngerusak interpreter lo
export function tSet(key: string, data: string): void { t.set(key, data); }
export function tGet(key: string): string { return t.get(key); }
export function tHas(key: string): bool { return t.has(key); }
export function tDelete(key: string): void { t.delete(key); }
export function tClear(): void { t.clear(); }
export function tConcat(keyA: string, keyB: string, newKey: string): void {
  t.set(newKey, t.get(keyA) + t.get(keyB));
}