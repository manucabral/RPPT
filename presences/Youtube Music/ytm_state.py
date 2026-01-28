"""Manejo de estado para el worker de YouTube Music."""

import time
from typing import Optional, Dict, Any
from src.runtime import Page
from src.logger import get_logger

logger = get_logger("youtube_music.state")


class YouTubeMusicState:
    """Gestiona el estado y caché para el presence de YouTube Music."""

    def __init__(self, update_interval: float = 5.0):
        self.last_media_session: Optional[Dict[str, Any]] = None
        self.last_update_time: float = 0
        self.update_interval: float = update_interval

        # Gestión de conexión
        self.connected_page: Optional[Page] = None
        self.last_page_id: Optional[str] = None

    def should_update_rpc(self, new_media: Optional[Dict[str, Any]]) -> bool:
        """
        Determina si se debe actualizar el RPC.

        Returns:
            True si el RPC debe actualizarse
        """
        # Siempre actualizar si no hay datos previos
        if self.last_media_session is None:
            return True

        # Actualizar si la media cambió
        if new_media != self.last_media_session:
            return True

        return False

    def mark_update(self, media_session: Optional[Dict[str, Any]]) -> None:
        """Marca que se envió una actualización de RPC."""
        self.last_media_session = media_session
        self.last_update_time = time.time()

    def select_best_page(self, pages: Dict[str, Page]) -> Optional[Page]:
        """
        Selecciona la mejor página para conectarse, prefiriendo la anterior.

        Returns:
            Mejor página para conectar, o None si no hay páginas
        """
        if not pages:
            return None

        # Preferir página previamente conectada si aún está disponible
        if self.last_page_id and self.last_page_id in pages:
            page = pages[self.last_page_id]
            logger.debug(
                "Reutilizando página previamente conectada %s", self.last_page_id
            )
            return page

        # Sino, tomar la última página (más reciente)
        pages_list = list(pages.values())
        selected = pages_list[-1]
        logger.debug("Seleccionando nueva página %s", selected.id)
        return selected

    def update_connected_page(self, page: Page) -> None:
        """Actualiza la página conectada."""
        self.connected_page = page
        self.last_page_id = page.id

    def cleanup(self) -> None:
        """Limpia recursos."""
        if self.connected_page:
            try:
                self.connected_page.close()
            except Exception:
                logger.debug("Error cerrando página durante cleanup", exc_info=True)
        self.connected_page = None
        self.last_page_id = None
