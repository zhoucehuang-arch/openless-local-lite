// Modal — centered backdrop + panel shell shared by local overlay UI.
//
// 动画沿用 global.css 的 ol-modal-backdrop-in / ol-modal-card-in（纯 opacity +
// transform，不碰 blur），与设置弹窗保持一致。

import type { ReactNode } from 'react';

interface ModalProps {
  children: ReactNode;
  onClose: () => void;
  /** 默认 50；多层叠加时（如登录弹窗叠在「我的发布」之上）传更大的值。 */
  zIndex?: number;
  /** 卡片宽度，默认 'min(560px, 100%)'。 */
  width?: string;
}

export function Modal({ children, onClose, zIndex = 50, width = 'min(560px, 100%)' }: ModalProps) {
  return (
    <div
      onClick={onClose}
      style={{
        position: 'fixed',
        inset: 0,
        background: 'rgba(0,0,0,0.22)',
        display: 'grid',
        placeItems: 'center',
        zIndex,
        padding: 20,
        animation: 'ol-modal-backdrop-in 0.18s var(--ol-motion-soft)',
      }}
    >
      <div
        onClick={e => e.stopPropagation()}
        style={{
          width,
          maxHeight: '85vh',
          overflow: 'auto',
          borderRadius: 16,
          background: 'var(--ol-surface)',
          border: '0.5px solid var(--ol-line-strong)',
          boxShadow: '0 18px 42px rgba(0,0,0,0.18)',
          padding: 22,
          animation: 'ol-modal-card-in 0.24s var(--ol-motion-spring)',
        }}
      >
        {children}
      </div>
    </div>
  );
}
