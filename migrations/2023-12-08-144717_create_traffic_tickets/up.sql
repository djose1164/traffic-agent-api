-- Your SQL goes here
CREATE TABLE traffic_tickets(
    id INT PRIMARY KEY AUTO_INCREMENT,
    name VARCHAR(100) NOT NULL UNIQUE,
    description VARCHAR(256)
);

INSERT INTO traffic_tickets(name, description)
VALUES
    ('Exceso de velocidad', 'Multa por rebasar los límites de velocidad establecidos en una zona determinada.'),
    ('No utilizar cinturón de seguridad', 'Multa por no llevar puesto el cinturón de seguridad mientras se conduce.'),
    ('Uso de teléfono móvil', 'Multa por hablar o enviar mensajes de texto con un teléfono móvil mientras se conduce.'),
    ('Conducir en estado de ebriedad', 'Multa por operar un vehículo bajo la influencia del alcohol.'),
    ('Violación de señales de tránsito', 'Multa por desobedecer señales de tránsito, como semáforos, señales de alto, o señales de dirección.'),
    ('No respetar las normas de adelantamiento', 'Multa por realizar adelantamientos en lugares prohibidos o de manera peligrosa.'),
    ('Estacionamiento indebido', 'Multa por estacionar en lugares no permitidos o en áreas reservadas.'),
    ('Falta de documentos', 'Multa por no llevar consigo la documentación obligatoria, como la licencia de conducir o la tarjeta de circulación.'),
    ('No ceder el paso en intersecciones', 'Multa por no dar prioridad adecuada en intersecciones o al ingresar a rotondas.'),
    ('No utilizar casco en motocicletas', 'Multa por no llevar puesto el casco de seguridad mientras se conduce una motocicleta.');