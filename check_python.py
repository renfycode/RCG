import time

def check_blazing():
    count = 0
    time_limit = 2  # 2 секунды
    start = time.time()  # Получаем текущее время в секундах с плавающей точкой

    while time.time() - start < time_limit:  # Проверяем, не прошло ли 2 секунды
        count += 1

    print(f"count: {count}")

# Вызов функции
check_blazing()


51728216
78690321
