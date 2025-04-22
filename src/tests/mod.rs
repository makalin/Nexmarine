#[cfg(test)]
mod tests {
    use super::super::protocol::{SeabindMessage, SeabindValue, ToSeabind};
    use super::super::config::{Config, InputConfig, OutputConfig};
    use std::path::PathBuf;
    use tokio::sync::broadcast;

    #[test]
    fn test_config_creation() {
        let config = Config {
            input: InputConfig {
                source: String::from("/dev/ttyUSB0"),
                format: String::from("nmea0183"),
            },
            output: OutputConfig {
                websocket: true,
                rest_api: true,
                seabind_log: Some(PathBuf::from("log.sbnd")),
            },
        };

        assert_eq!(config.input.source, "/dev/ttyUSB0");
        assert_eq!(config.input.format, "nmea0183");
        assert!(config.output.websocket);
        assert!(config.output.rest_api);
        assert_eq!(config.output.seabind_log.unwrap(), PathBuf::from("log.sbnd"));
    }

    #[test]
    fn test_seabind_message_creation() {
        let msg = SeabindMessage::new(
            String::from("nav.position.latitude"),
            SeabindValue::Number(45.5),
        ).with_unit("degrees")
         .with_source("GPS");

        assert_eq!(msg.path, "nav.position.latitude");
        match msg.value {
            SeabindValue::Number(n) => assert_eq!(n, 45.5),
            _ => panic!("Expected Number value"),
        }
        assert_eq!(msg.unit.unwrap(), "degrees");
        assert_eq!(msg.source.unwrap(), "GPS");
    }

    #[tokio::test]
    async fn test_broadcast_channel() {
        let (tx, mut rx1) = broadcast::channel(100);
        let mut rx2 = tx.subscribe();

        let msg = SeabindMessage::new(
            String::from("test.value"),
            SeabindValue::Number(123.45),
        );

        tx.send(msg.clone()).unwrap();

        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        assert_eq!(received1.path, "test.value");
        assert_eq!(received2.path, "test.value");
    }
} 