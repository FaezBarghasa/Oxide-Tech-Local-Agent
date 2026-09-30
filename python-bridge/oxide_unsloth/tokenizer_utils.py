"""
Oxide Tokenizer Utilities & Chat Template Standardizer
"""

from typing import Dict, Any, List, Optional, Union

CHAT_TEMPLATES = {
    "chatml": (
        "{% for message in messages %}"
        "{{'<|im_start|>' + message['role'] + '\n' + message['content'] + '<|im_end|>\n'}}"
        "{% endfor %}"
        "{% if add_generation_prompt %}"
        "{{ '<|im_start|>assistant\n' }}"
        "{% endif %}"
    ),
    "llama-3": (
        "{% set loop_messages = messages %}"
        "{% for message in loop_messages %}"
        "{{ '<|start_header_id|>' + message['role'] + '<|end_header_id|>\n\n' + message['content'] | trim + '<|eot_id|>' }}"
        "{% endfor %}"
        "{% if add_generation_prompt %}"
        "{{ '<|start_header_id|>assistant<|end_header_id|>\n\n' }}"
        "{% endif %}"
    ),
    "qwen-2.5": (
        "{% for message in messages %}"
        "{{'<|im_start|>' + message['role'] + '\n' + message['content'] + '<|im_end|>\n'}}"
        "{% endfor %}"
        "{% if add_generation_prompt %}"
        "{{ '<|im_start|>assistant\n' }}"
        "{% endif %}"
    ),
    "deepseek-r1": (
        "{% for message in messages %}"
        "{% if message['role'] == 'user' %}"
        "{{ '<｜User｜>' + message['content'] }}"
        "{% elif message['role'] == 'assistant' %}"
        "{{ '<｜Assistant｜>' + message['content'] + '<｜end of sentence｜>' }}"
        "{% endif %}"
        "{% endfor %}"
        "{% if add_generation_prompt %}"
        "{{ '<｜Assistant｜><think>\n' }}"
        "{% endif %}"
    ),
}


def get_chat_template(
    tokenizer: Any,
    chat_template: str = "chatml",
    mapping: Optional[Dict[str, str]] = None,
) -> Any:
    """Sets standard high-performance chat template on tokenizer."""
    template_str = CHAT_TEMPLATES.get(chat_template.lower(), CHAT_TEMPLATES["chatml"])
    if hasattr(tokenizer, "chat_template"):
        tokenizer.chat_template = template_str
    return tokenizer


def standardize_sharegpt(
    dataset: List[Dict[str, Any]],
    aliases: Optional[Dict[str, str]] = None,
) -> List[Dict[str, Any]]:
    """Standardizes heterogeneous conversation roles (human/gpt, user/assistant, bot)."""
    role_map = {
        "human": "user",
        "gpt": "assistant",
        "bot": "assistant",
        "system": "system",
        "user": "user",
        "assistant": "assistant",
    }
    if aliases:
        role_map.update(aliases)

    standardized = []
    for entry in dataset:
        convs = entry.get("conversations", entry.get("messages", []))
        new_convs = []
        for c in convs:
            r = c.get("from", c.get("role", "user"))
            val = c.get("value", c.get("content", ""))
            new_convs.append({
                "role": role_map.get(r.lower(), "user"),
                "content": val,
            })
        standardized.append({"conversations": new_convs})

    return standardized
