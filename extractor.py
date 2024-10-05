import re


def clear_a_and_tag(line):
    cleaned_text = re.sub(r'<a[^>]*>.*?</a>', '', line)
    return re.sub(r'<[^>]*>', '', cleaned_text)


def check_nav_link(line):
    if re.search(r'<a[^>]*>.*?</a>', line):
        cleaned_text = clear_a_and_tag(line)
        return len(cleaned_text.strip()) < 1
    return False


def check_not_pure_tag(line):
    if line:
        cleaned_text = re.sub(r'<[^>]*>', '', line)
        return len(cleaned_text.strip()) > 0


def window_group(marked, window_size=2):
    groups = []
    current = [marked[0], ]
    for n in marked[1:]:
        if n - window_size <= current[-1]:
            current.append(n)
        else:
            groups.append(current)
            current = [n,]
    return groups


def process(html: str) -> str:
    body_content = re.search(r'<body[^>]*>(.*?)</body>', html, re.DOTALL)
    body_content = body_content.group(1) if body_content else None
    cleaned_content = re.sub(r'<script[^>]*>.*?</script>', '', body_content, flags=re.DOTALL)
    cleaned_content = re.sub(r'<style[^>]*>.*?</style>', '', cleaned_content, flags=re.DOTALL)
    cleaned_map = {}
    marked = []
    for line_no, line in enumerate(cleaned_content.split("\n")):
        if not check_nav_link(line):
            if check_not_pure_tag(line):
                marked.append(line_no)
                cleaned_map[line_no] = clear_a_and_tag(line).strip()
    groups = window_group(marked)
    max_weight = 0
    max_weight_idx = -1
    for gp_no, gp in enumerate(groups):
        weight = sum([len(cleaned_map[no]) for no in gp])
        if max_weight<= weight:
            max_weight = weight
            max_weight_idx = gp_no
        print("group:", gp, "  wight:", weight)
    print("max weight:", max_weight, "  max weight idx:", max_weight_idx)
    return "\n".join([cleaned_map[line_no] for line_no in groups[max_weight_idx]])



def test():
    import requests
    r = requests.get("https://quanben.io/n/wenyishidai/1.html")
    #r = requests.get("https://www.27k.net/read/223814/78262627.html")
    content = process(r.text)
    print(content)


if __name__ == "__main__":
    test()
