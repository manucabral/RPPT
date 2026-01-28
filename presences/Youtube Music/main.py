import time
from typing import Optional, Any, Dict
from src.rpc import ActivityType, ClientRPC
from src.runtime import Runtime, Page
from src.logger import logger
from .ytm_state import YouTubeMusicState

def on_load(rpc: ClientRPC):
    rpc.update(
        state="Listening to YouTube Music",
        details=None,
        activity_type=ActivityType.LISTENING,
        start_time=int(time.time()),
        end_time=None,
        large_image="youtube_music_logo",
        large_text="YouTube Music",
        small_image=None,
        small_text=None,
        buttons=[],
    )


def get_ytm_pages(runtime: Runtime) -> Dict[str, Page]:
    """Obtiene todas las páginas de YouTube Music."""
    ytm_pages: Dict[str, Page] = {}
    for page in runtime.pages:
        if not page.url or not page.id:
            continue
        if "music.youtube.com" in page.url:
            ytm_pages[page.id] = page
    return ytm_pages


def format_rpc_data(media_session: Dict[str, Any]) -> Dict[str, Any]:
    """Formatea los datos de media session para actualización de RPC."""
    title = media_session.get("title")
    artist = media_session.get("artist")
    album = media_session.get("album")

    # details -> shown as main line; state -> smaller secondary line
    details = title if title else "Escuchando música"
    state = artist if artist else None
    if album:
        state = f"{state} | {album}" if state else f"{album}"

    return {
        "state": state,
        "details": details,
        "activity_type": ActivityType.LISTENING,
        "start_time": int(time.time()),
        "end_time": None,
        "large_image": "youtube_music_logo",
        "large_text": "YouTube Music",
        "small_image": None,
        "small_text": None,
        "buttons": [],
    }


def main(rpc: ClientRPC, runtime: Optional[Runtime], stop_event: Any):
    """Loop principal del worker de YouTube Music."""
    if runtime is None:
        raise RuntimeError("Runtime is required for YouTube Music presence")

    logger.info("Presence started")
    on_load(rpc)

    state = YouTubeMusicState(update_interval=5.0)
    was_idle = False  # Track si estamos en estado idle

    try:
        while not stop_event.is_set():
            # Obtener páginas disponibles de YouTube Music
            pages = get_ytm_pages(runtime)

            if len(pages) == 0:
                # No hay pestañas de YouTube Music - actualizar a idle
                if not was_idle:
                    logger.info(
                        "No hay pestañas de YouTube Music - actualizando a idle"
                    )
                    payload = {
                        "state": "Sin pestañas abiertas",
                        "details": "Esperando...",
                        "activity_type": ActivityType.LISTENING,
                        "start_time": None,
                        "end_time": None,
                        "large_image": "youtube_music_logo",
                        "large_text": "YouTube Music",
                        "small_image": None,
                        "small_text": None,
                        "buttons": [],
                    }
                    logger.debug("RPC update (idle) payload: %s", payload)
                    rpc.update(**payload)
                    state.cleanup()  # Limpiar conexión si había
                    was_idle = True

                logger.debug(
                    "No se detectaron páginas de YouTube Music. Esperando 5 segundos."
                )
                stop_event.wait(5)
                continue

            # Hay pestañas, resetear flag de idle
            was_idle = False

            # Si ya tenemos una página conectada, verificar que todavía existe
            if state.connected_page and state.last_page_id:
                # Si la página aún está en la lista del runtime
                if state.last_page_id in pages:
                    # Usar la página conectada que ya tenemos (mantiene la conexión)
                    page = state.connected_page
                    logger.debug("Reutilizando página conectada: %s", page.id)
                else:
                    # La página ya no existe, limpiar y seleccionar nueva
                    logger.debug("Página anterior ya no existe, seleccionando nueva")
                    state.cleanup()
                    page = state.select_best_page(pages)
            else:
                # No hay página conectada, seleccionar una
                page = state.select_best_page(pages)

            if page is None:
                logger.debug("No se encontró una página válida. Esperando 5 segundos.")
                stop_event.wait(5)
                continue

            logger.debug("Target: %s (id=%s)", page.title, page.id)

            # Conectar si es necesario (reutiliza conexión existente si está viva)
            try:
                page.connect_if_needed(timeout=3.0)
                # Solo actualizar si es una página nueva
                if state.connected_page != page:
                    state.update_connected_page(page)
            except Exception as exc:
                logger.warning("Error al conectar a página %s: %s", page.id, exc)
                state.cleanup()
                stop_event.wait(5)
                continue

            # Leer media session
            media_session = None
            try:
                media_session = page.get_media_session(timeout=3.0)
            except Exception as exc:
                logger.warning("Error leyendo media session de %s: %s", page.url, exc)
                stop_event.wait(5)
                continue

            if media_session is None:
                logger.debug("No hay media session disponible. Esperando 5 segundos.")
                stop_event.wait(5)
                continue

            # Actualizar RPC solo si es necesario (media cambió)
            if state.should_update_rpc(media_session):
                logger.info("Actualizando RPC: %s", media_session)
                rpc_data = format_rpc_data(media_session)
                logger.debug("RPC update (media) payload: %s", rpc_data)
                rpc.update(**rpc_data)
                state.mark_update(media_session)
            else:
                logger.debug("No se necesita actualización de RPC, media sin cambios")

            stop_event.wait(5)
    finally:
        state.cleanup()
        logger.info("Stopping")