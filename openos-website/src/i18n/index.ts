import en from './en';
import zh from './zh';

export type Locale = 'en' | 'zh';

const translations = { en, zh } as const;

export function useTranslations(locale: Locale) {
  return translations[locale] || translations.en;
}

export function getLocaleFromPath(path: string): Locale {
  if (path.startsWith('/zh') || path.startsWith('/zh/')) {
    return 'zh';
  }
  return 'en';
}

export function getLocalizedPath(path: string, locale: Locale): string {
  // Remove existing locale prefix
  let cleanPath = path;
  if (cleanPath.startsWith('/zh')) {
    cleanPath = cleanPath.substring(3) || '/';
  }

  // Add locale prefix for non-English locales
  if (locale === 'zh') {
    return `/zh${cleanPath}`;
  }

  return cleanPath;
}

export { en, zh };
