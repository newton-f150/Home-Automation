-- Sample sensor readings
INSERT INTO sensor_readings (
    temperature,
    pressure,
    air_quality,
    light,
    recorded_at
) VALUES
(24.5, 1012.3, 42.1, 350.0, NOW() - INTERVAL '30 minutes'),
(24.8, 1012.5, 41.8, 420.0, NOW() - INTERVAL '25 minutes'),
(25.1, 1012.6, 40.9, 510.0, NOW() - INTERVAL '20 minutes'),
(25.6, 1012.8, 39.5, 620.0, NOW() - INTERVAL '15 minutes'),
(26.0, 1013.0, 38.7, 700.0, NOW() - INTERVAL '10 minutes'),
(26.2, 1013.1, 37.9, 780.0, NOW() - INTERVAL '5 minutes'),
(26.4, 1013.2, 37.2, 820.0, NOW());

-- Sample command history
INSERT INTO command_log (command, status, sent_at)
VALUES
('LED_ON', 'ok', NOW() - INTERVAL '20 minutes'),
('FAN_ON', 'ok', NOW() - INTERVAL '15 minutes'),
('LED_OFF', 'ok', NOW() - INTERVAL '10 minutes'),
('FAN_OFF', 'failed', NOW() - INTERVAL '8 minutes'),
('BUZZER_ON', 'sent', NOW() - INTERVAL '2 minutes');

-- Update the device status row
UPDATE device_status
SET
    esp32_online = TRUE,
    mqtt_connected = TRUE,
    last_seen = NOW(),
    updated_at = NOW()
WHERE id = 1;