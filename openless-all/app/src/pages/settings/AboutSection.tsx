// About: local version information and personalization.

import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { Row } from '../../components/ui/Row';
import { APP_VERSION_LABEL } from '../../lib/appVersion';
import { readFontScale, setFontScale, type FontScaleId } from '../../lib/fontScale';
import { Card } from '../_atoms';
import { SectionTitle } from './shared';

export function AboutSection() {
  const { t } = useTranslation();

  return (
    <>
      {/* Version */}
      <Card>
        <div style={{ display: 'flex', alignItems: 'center', gap: 14 }}>
          <img
            src="AppIcon.png"
            alt=""
            style={{ width: 56, height: 56, borderRadius: 13, boxShadow: '0 4px 10px rgba(0,0,0,.10), 0 0 0 0.5px rgba(0,0,0,.06)' }}
          />
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ fontSize: 17, fontWeight: 600 }}>OpenLess Local</div>
            <div style={{ fontSize: 12, color: 'var(--ol-ink-3)', marginTop: 2 }}>
              {t('modal.about.tagline')} · {APP_VERSION_LABEL}
            </div>
          </div>
        </div>
      </Card>

      {/* Personalization */}
      <Card>
        <SectionTitle>{t('modal.sections.personalize')}</SectionTitle>
        <FontSizeRow />
      </Card>

    </>
  );
}

function FontSizeRow() {
  const { t } = useTranslation();
  const [fontScale, setFontScaleState] = useState<FontScaleId>(() => readFontScale());
  const applyFontScaleChoice = (next: FontScaleId) => {
    setFontScaleState(next);
    setFontScale(next);
  };
  const fontOptions: Array<[FontScaleId, string]> = [
    ['small', t('modal.personalize.fontSmall')],
    ['medium', t('modal.personalize.fontMedium')],
    ['large', t('modal.personalize.fontLarge')],
  ];
  return (
    <Row label={t('modal.personalize.font')}>
      <div style={{ display: 'flex', gap: 4, padding: 2, background: 'rgba(0,0,0,0.04)', borderRadius: 8 }}>
        {fontOptions.map(([id, label]) => {
          const selected = fontScale === id;
          return (
            <button
              key={id}
              onClick={() => applyFontScaleChoice(id)}
              style={{
                minWidth: 64,
                height: 28,
                border: 0,
                borderRadius: 6,
                background: selected ? '#fff' : 'transparent',
                color: selected ? 'var(--ol-ink)' : 'var(--ol-ink-3)',
                fontFamily: 'inherit',
                fontSize: 12,
                fontWeight: selected ? 600 : 500,
                cursor: 'default',
                boxShadow: selected ? '0 1px 2px rgba(0,0,0,.06), 0 0 0 0.5px rgba(0,0,0,.06)' : 'none',
                transition: 'background 0.16s var(--ol-motion-quick), color 0.16s var(--ol-motion-quick), box-shadow 0.18s var(--ol-motion-soft)',
                padding: '0 12px',
              }}
            >
              {label}
            </button>
          );
        })}
      </div>
    </Row>
  );
}
