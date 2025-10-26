mod mqtt_client;

use gpio_cdev::{Chip, EventRequestFlags, LineRequestFlags};
use std::io::Read;
use std::os::fd::AsRawFd;

use crate::mqtt_client::MqttClient;
use apator_wmbus_mqtt::cc1101::cc1101::CC1101;
use apator_wmbus_mqtt::cc1101::cc1101_config::CC1101Config;
use apator_wmbus_mqtt::cc1101::cc1101_enums::{CommandStrobe, ConfigurationRegisterAddress, StatusRegisterAddress};
use apator_wmbus_mqtt::cc1101::cc1101_registers::*;
use apator_wmbus_mqtt::config::{GPIO_CHIP, GPIO_LINE, SPI_DEVICE, SPI_RATE};
use apator_wmbus_mqtt::spi_device::SpiDevice;
use apator_wmbus_mqtt::tprintln;
use apator_wmbus_mqtt::utils::bytes_to_hex;
use apator_wmbus_mqtt::wmbus::decode_3of6;
use apator_wmbus_mqtt::wmbus::wmbus_decode::decode_packet;

fn main() {
    unsafe {
        let priority = -20;

        if libc::setpriority(libc::PRIO_PROCESS, 0, priority) != 0 {
            eprintln!("Failed to set the nice value. Error code: {}", std::io::Error::last_os_error());
        }
    }

    let mut cc1101_cfg = CC1101Config::new();
    cc1101_cfg.write_config(ConfigurationRegisterAddress::IOCFG2, IOCFG2::from(0x06));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::IOCFG1, IOCFG1::from(0x2E));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::IOCFG0, IOCFG0::from(0x00)); // GD0 = RX not empty
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FIFOTHR, FIFOTHR::from(0x0)); // 4 bytes in RX queue triggers GD0
    cc1101_cfg.write_SYNC(0x543d);
    cc1101_cfg.write_config(ConfigurationRegisterAddress::PKTLEN, PKTLEN::new().with_PACKET_LENGTH(0xff));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::PKTCTRL1, PKTCTRL1::from(0x0));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::PKTCTRL0, PKTCTRL0::from(0x0));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::ADDR, ADDR::from(0x0));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::CHANNR, CHANNR::from(0x0));
    cc1101_cfg.write_FREQ_IF(200000);
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSCTRL0, FSCTRL0::from(0x0));
    cc1101_cfg.write_FREQ(868_950_000);
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MDMCFG4, MDMCFG4::from(0x5C));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MDMCFG3, MDMCFG3::from(0x4));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MDMCFG2, MDMCFG2::from(0x6));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MDMCFG1, MDMCFG1::from(0x22));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MDMCFG0, MDMCFG0::from(0xF8));
    cc1101_cfg.write_DEVIATN(50000);
    // cc1101_cfg.write_config(ConfigurationRegisterAddress::DEVIATN, DEVIATN::from(0x44));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MCSM2, MCSM2::from(0x7));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MCSM1, MCSM1::from(0x00));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::MCSM0, MCSM0::from(0x18));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FOCCFG, FOCCFG::from(0x2E));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::BSCFG, BSCFG::from(0xBF));
    cc1101_cfg.write_config(
        ConfigurationRegisterAddress::AGCTRL2,
        AGCCTRL2::new()
            .with_MAX_DVGA_GAIN(1)
            .with_MAX_LNA_GAIN(0)
            .with_MAGN_TARGET(5),
    );
    cc1101_cfg.write_config(
        ConfigurationRegisterAddress::AGCTRL1,
        AGCCTRL1::new()
            .with_AGC_LNA_PRIORITY(0)
            .with_CARRIER_SENSE_REL_THR(0)
            .with_CARRIER_SENSE_ABS_THR(0),
    );
    cc1101_cfg.write_config(ConfigurationRegisterAddress::AGCTRL0, AGCCTRL0::from(0xB5));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::WOREVT1, WOREVT1::from(0x87));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::WOREVT0, WOREVT0::from(0x6B));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::WORCTRL, WORCTRL::from(0xFB));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FREND1, FREND1::from(0xB6));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FREND0, FREND0::from(0x10));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSCAL3, FSCAL3::from(0xEA));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSCAL2, FSCAL2::from(0x2A));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSCAL1, FSCAL1::from(0x0));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSCAL0, FSCAL0::from(0x1F));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::RCCTRL1, RCCTRL1::from(0x41));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::RCCTRL0, RCCTRL0::from(0x0));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::FSTEST, FSTEST::from(0x59));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::PTEST, PTEST::from(0x7F));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::AGCTEST, AGCTEST::from(0x3F));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::TEST2, TEST2::from(0x81));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::TEST1, TEST1::from(0x35));
    cc1101_cfg.write_config(ConfigurationRegisterAddress::TEST0, TEST0::from(0x9));

    let mut chip = Chip::new(GPIO_CHIP).unwrap();
    let line = chip.get_line(GPIO_LINE).unwrap();

    let spi = SpiDevice::create(SPI_DEVICE, SPI_RATE);

    let mut cc1101 = CC1101::new(spi);

    cc1101.command(CommandStrobe::SRES);

    tprintln!("IOCFG2 = {}", cc1101.read_config(ConfigurationRegisterAddress::IOCFG2));
    cc1101.apply_config(cc1101_cfg);

    cc1101.command(CommandStrobe::SFRX);
    cc1101.command(CommandStrobe::SRX);

    let mqtt = MqttClient::connect();

    tprintln!("start");
    for _event in line
        .events(LineRequestFlags::INPUT, EventRequestFlags::RISING_EDGE, "apator_wmbus_mqtt")
        .unwrap()
    {
        let mut rxbytes = cc1101.read_status(StatusRegisterAddress::RXBYTES) & 0x7f;

        if rxbytes > 0 {
            let mut parts = Vec::new();
            parts.reserve(20);

            while rxbytes > 0 {
                let pck = cc1101.read_registers_burst(0x3F, rxbytes);
                parts.push(pck);

                rxbytes = cc1101.read_status(StatusRegisterAddress::RXBYTES) & 0x7f;
            }

            for pck in &parts {
                tprintln!("part ({}): {:?}", pck.len(), pck);
            }

            let mut rssi = cc1101.read_status(StatusRegisterAddress::RSSI) as f32;
            if rssi >= 128.0 {
                rssi = (rssi - 256.0) / 2.0 - 74.0;
            } else {
                rssi = rssi / 2.0 - 74.0;
            }
            let full_packet: Vec<u8> = parts
                .iter()
                .flat_map(|x| x.iter().cloned())
                .collect();

            tprintln!("full_packet ({}, rssi: {})", full_packet.len(), rssi);

            cc1101.command(CommandStrobe::SFRX);
            cc1101.command(CommandStrobe::SRX);

            process_packet(&mqtt, &full_packet);
        }
    }

    loop {}
}

fn process_packet(mqtt: &MqttClient, received_packet: &Vec<u8>) {
    let data = decode_3of6::decode_3of6(&received_packet);
    if data.is_empty() {
        return;
    }

    let telegram = decode_packet(&data);
    match telegram {
        Ok(tg) => {
            mqtt.publish(
                format!("apator/{}/volume", tg.device_identifier).as_str(),
                tg.total_water_volume.to_string().as_str(),
                true,
            );

            tprintln!("telegram: {} = {:?}", tg.device_identifier, tg.total_water_volume);
        }
        Err(e) => {
            tprintln!("err: {}", e);
        }
    }
}
