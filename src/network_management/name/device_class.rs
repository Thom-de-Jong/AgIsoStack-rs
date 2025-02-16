// Copyright 2023 Raven Industries inc.

use super::IndustryGroup;

/// Enum containing all Device Classes.
/// Some Device classes belong to multiple Industry Groups.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::{IndustryGroup, DeviceClass};
/// let device_class: DeviceClass = DeviceClass::Fertilizers;
///
/// assert_eq!(device_class, DeviceClass::from((5, IndustryGroup::AgriculturalAndForestryEquipment)));
/// assert_eq!(device_class, (5, IndustryGroup::AgriculturalAndForestryEquipment).into());
/// assert_eq!(5, u8::from(device_class));
/// assert_eq!(5_u8, device_class.into());
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum DeviceClass {
    // Shared
    #[default]
    NotAvailable,
    NonSpecificSystem(IndustryGroup),
    Tractor(IndustryGroup),

    // On Highway Equipment
    Trailer,

    // Agricultural And Forestry Equipment
    Tillage,
    SecondaryTillage,
    PlantersOrSeeders,
    Fertilizers,
    Sprayers,
    Harvesters,
    RootHarvesters,
    Forage,
    Irrigation,
    TransportOrTrailer,
    FarmYardOperations,
    PoweredAuxiliaryDevices,
    SpecialCrops,
    EarthWork,
    Skidder,
    SensorSystems,
    TimberHarvesters,
    Forwarders,
    TimberLoaders,
    TimberProcessingMachines,
    Mulchers,
    UtilityVehicles,
    SlurryOrManureApplicators,
    FeedersOrMixers,
    Weeders,

    // Construction Equipment
    SkidSteerLoader,
    ArticulatedDumpTruck,
    Backhoe,
    Crawler,
    Excavator,
    Forklift,
    FourWheelDriveLoader,
    Grader,
    MillingMachine,
    RecyclerAndSoilStabilizer,
    BindingAgentSpreader,
    Paver,
    Feeder,
    ScreeningPlant,
    Stacker,
    Roller,
    Crusher,

    // Marine Equipment
    SystemTools,
    SafetySystems,
    Gateway,
    PowerManagementAndLightingSystems,
    Steeringsystems,
    NavigationSystems,
    CommunicationsSystems,
    InstrumentationOrGeneralSystems,
    EnvironmentalSystems,
    DeckCargoAndFishingEquipmentSystems,

    // Industrial Process Control
    IndustrialProcessControlStationary,
}

/// Display the Device Class name.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::DeviceClass;
///
/// assert_eq!("Fertilizers", format!("{}", DeviceClass::Fertilizers));
/// ```
impl core::fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Convert a `DeviceClass` into a u8.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::DeviceClass;
///
/// assert_eq!(5, u8::from(DeviceClass::Fertilizers));
/// assert_eq!(5_u8, DeviceClass::Fertilizers.into());
/// ```
impl From<DeviceClass> for u8 {
    fn from(value: DeviceClass) -> Self {
        match value {
            // Shared
            DeviceClass::NotAvailable => 127,
            DeviceClass::NonSpecificSystem(_) => 0,
            DeviceClass::Tractor(_) => 1,

            // On Highway Equipment
            DeviceClass::Trailer => 2,

            // Agricultural And Forestry Equipment
            DeviceClass::Tillage => 2,
            DeviceClass::SecondaryTillage => 3,
            DeviceClass::PlantersOrSeeders => 4,
            DeviceClass::Fertilizers => 5,
            DeviceClass::Sprayers => 6,
            DeviceClass::Harvesters => 7,
            DeviceClass::RootHarvesters => 8,
            DeviceClass::Forage => 9,
            DeviceClass::Irrigation => 10,
            DeviceClass::TransportOrTrailer => 11,
            DeviceClass::FarmYardOperations => 12,
            DeviceClass::PoweredAuxiliaryDevices => 13,
            DeviceClass::SpecialCrops => 14,
            DeviceClass::EarthWork => 15,
            DeviceClass::Skidder => 16,
            DeviceClass::SensorSystems => 17,
            DeviceClass::TimberHarvesters => 19,
            DeviceClass::Forwarders => 20,
            DeviceClass::TimberLoaders => 21,
            DeviceClass::TimberProcessingMachines => 22,
            DeviceClass::Mulchers => 23,
            DeviceClass::UtilityVehicles => 24,
            DeviceClass::SlurryOrManureApplicators => 25,
            DeviceClass::FeedersOrMixers => 26,
            DeviceClass::Weeders => 27,

            // Construction Equipment
            DeviceClass::SkidSteerLoader => 1,
            DeviceClass::ArticulatedDumpTruck => 2,
            DeviceClass::Backhoe => 3,
            DeviceClass::Crawler => 4,
            DeviceClass::Excavator => 5,
            DeviceClass::Forklift => 6,
            DeviceClass::FourWheelDriveLoader => 7,
            DeviceClass::Grader => 8,
            DeviceClass::MillingMachine => 9,
            DeviceClass::RecyclerAndSoilStabilizer => 10,
            DeviceClass::BindingAgentSpreader => 11,
            DeviceClass::Paver => 12,
            DeviceClass::Feeder => 13,
            DeviceClass::ScreeningPlant => 14,
            DeviceClass::Stacker => 15,
            DeviceClass::Roller => 16,
            DeviceClass::Crusher => 17,

            // Marine Equipment
            DeviceClass::SystemTools => 10,
            DeviceClass::SafetySystems => 20,
            DeviceClass::Gateway => 25,
            DeviceClass::PowerManagementAndLightingSystems => 30,
            DeviceClass::Steeringsystems => 40,
            DeviceClass::NavigationSystems => 60,
            DeviceClass::CommunicationsSystems => 70,
            DeviceClass::InstrumentationOrGeneralSystems => 80,
            DeviceClass::EnvironmentalSystems => 90,
            DeviceClass::DeckCargoAndFishingEquipmentSystems => 100,

            // Industrial Process Control
            DeviceClass::IndustrialProcessControlStationary => 0,
        }
    }
}

/// Convert a u8 and `IndustryGroup` into a `DeviceClass`.
/// 
/// The `IndustryGroup` is needed becouse a u8 can represent multiple `DeviceClass`es depending on the `IndustryGroup`.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::{IndustryGroup, DeviceClass};
///
/// assert_eq!(DeviceClass::Fertilizers, DeviceClass::from((5, IndustryGroup::AgriculturalAndForestryEquipment)));
/// assert_eq!(DeviceClass::Fertilizers, (5, IndustryGroup::AgriculturalAndForestryEquipment).into());
/// ```
#[rustfmt::skip] // Skip formatting the lines inside the match statement
impl From<(u8, IndustryGroup)> for DeviceClass {
    fn from(value: (u8, IndustryGroup)) -> Self {
        match value {
            (0, IndustryGroup::IndustrialProcessControl) => DeviceClass::IndustrialProcessControlStationary,
            (0, ig) => DeviceClass::NonSpecificSystem(ig),

            (1, IndustryGroup::OnHighwayEquipment) => DeviceClass::Tractor(IndustryGroup::OnHighwayEquipment),
            (2, IndustryGroup::OnHighwayEquipment) => DeviceClass::Trailer,

            (1, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Tractor(IndustryGroup::AgriculturalAndForestryEquipment),
            (2, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Tillage,
            (3, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::SecondaryTillage,
            (4, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::PlantersOrSeeders,
            (5, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Fertilizers,
            (6, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Sprayers,
            (7, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Harvesters,
            (8, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::RootHarvesters,
            (9, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Forage,
            (10, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Irrigation,
            (11, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::TransportOrTrailer,
            (12, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::FarmYardOperations,
            (13, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::PoweredAuxiliaryDevices,
            (14, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::SpecialCrops,
            (15, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::EarthWork,
            (16, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Skidder,
            (17, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::SensorSystems,
            (19, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::TimberHarvesters,
            (20, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Forwarders,
            (21, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::TimberLoaders,
            (22, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::TimberProcessingMachines,
            (23, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Mulchers,
            (24, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::UtilityVehicles,
            (25, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::SlurryOrManureApplicators,
            (26, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::FeedersOrMixers,
            (27, IndustryGroup::AgriculturalAndForestryEquipment) => DeviceClass::Weeders,

            (1, IndustryGroup::ConstructionEquipment) => DeviceClass::SkidSteerLoader,
            (2, IndustryGroup::ConstructionEquipment) => DeviceClass::ArticulatedDumpTruck,
            (3, IndustryGroup::ConstructionEquipment) => DeviceClass::Backhoe,
            (4, IndustryGroup::ConstructionEquipment) => DeviceClass::Crawler,
            (5, IndustryGroup::ConstructionEquipment) => DeviceClass::Excavator,
            (6, IndustryGroup::ConstructionEquipment) => DeviceClass::Forklift,
            (7, IndustryGroup::ConstructionEquipment) => DeviceClass::FourWheelDriveLoader,
            (8, IndustryGroup::ConstructionEquipment) => DeviceClass::Grader,
            (9, IndustryGroup::ConstructionEquipment) => DeviceClass::MillingMachine,
            (10, IndustryGroup::ConstructionEquipment) => DeviceClass::RecyclerAndSoilStabilizer,
            (11, IndustryGroup::ConstructionEquipment) => DeviceClass::BindingAgentSpreader,
            (12, IndustryGroup::ConstructionEquipment) => DeviceClass::Paver,
            (13, IndustryGroup::ConstructionEquipment) => DeviceClass::Feeder,
            (14, IndustryGroup::ConstructionEquipment) => DeviceClass::ScreeningPlant,
            (15, IndustryGroup::ConstructionEquipment) => DeviceClass::Stacker,
            (16, IndustryGroup::ConstructionEquipment) => DeviceClass::Roller,
            (17, IndustryGroup::ConstructionEquipment) => DeviceClass::Crusher,

            (10, IndustryGroup::MarineEquipment) => DeviceClass::SystemTools,
            (20, IndustryGroup::MarineEquipment) => DeviceClass::SafetySystems,
            (25, IndustryGroup::MarineEquipment) => DeviceClass::Gateway,
            (30, IndustryGroup::MarineEquipment) => DeviceClass::PowerManagementAndLightingSystems,
            (40, IndustryGroup::MarineEquipment) => DeviceClass::Steeringsystems,
            (60, IndustryGroup::MarineEquipment) => DeviceClass::NavigationSystems,
            (70, IndustryGroup::MarineEquipment) => DeviceClass::CommunicationsSystems,
            (80, IndustryGroup::MarineEquipment) => DeviceClass::InstrumentationOrGeneralSystems,
            (90, IndustryGroup::MarineEquipment) => DeviceClass::EnvironmentalSystems,
            (100, IndustryGroup::MarineEquipment) => DeviceClass::DeckCargoAndFishingEquipmentSystems,

            _ => DeviceClass::NotAvailable,
        }
    }
}

/// Derive the `IndustryGroup` from a `DeviceClass`.
///
/// # Examples
///
/// ```rust
/// # use ag_iso_stack::network_management::name::{IndustryGroup, DeviceClass};
///
/// assert_eq!(IndustryGroup::AgriculturalAndForestryEquipment, IndustryGroup::from(DeviceClass::Fertilizers));
/// assert_eq!(IndustryGroup::AgriculturalAndForestryEquipment, DeviceClass::Fertilizers.into());
/// ```
impl From<DeviceClass> for IndustryGroup {
    fn from(value: DeviceClass) -> Self {
        match value {
            // Shared
            DeviceClass::NotAvailable => IndustryGroup::Global,
            DeviceClass::NonSpecificSystem(ig) | DeviceClass::Tractor(ig) => ig,

            // On Highway Equipment
            DeviceClass::Trailer => IndustryGroup::OnHighwayEquipment,

            // Agricultural And Forestry Equipment
            DeviceClass::Tillage
            | DeviceClass::SecondaryTillage
            | DeviceClass::PlantersOrSeeders
            | DeviceClass::Fertilizers
            | DeviceClass::Sprayers
            | DeviceClass::Harvesters
            | DeviceClass::RootHarvesters
            | DeviceClass::Forage
            | DeviceClass::Irrigation
            | DeviceClass::TransportOrTrailer
            | DeviceClass::FarmYardOperations
            | DeviceClass::PoweredAuxiliaryDevices
            | DeviceClass::SpecialCrops
            | DeviceClass::EarthWork
            | DeviceClass::Skidder
            | DeviceClass::SensorSystems
            | DeviceClass::TimberHarvesters
            | DeviceClass::Forwarders
            | DeviceClass::TimberLoaders
            | DeviceClass::TimberProcessingMachines
            | DeviceClass::Mulchers
            | DeviceClass::UtilityVehicles
            | DeviceClass::SlurryOrManureApplicators
            | DeviceClass::FeedersOrMixers
            | DeviceClass::Weeders => IndustryGroup::AgriculturalAndForestryEquipment,

            // Construction Equipment
            DeviceClass::SkidSteerLoader
            | DeviceClass::ArticulatedDumpTruck
            | DeviceClass::Backhoe
            | DeviceClass::Crawler
            | DeviceClass::Excavator
            | DeviceClass::Forklift
            | DeviceClass::FourWheelDriveLoader
            | DeviceClass::Grader
            | DeviceClass::MillingMachine
            | DeviceClass::RecyclerAndSoilStabilizer
            | DeviceClass::BindingAgentSpreader
            | DeviceClass::Paver
            | DeviceClass::Feeder
            | DeviceClass::ScreeningPlant
            | DeviceClass::Stacker
            | DeviceClass::Roller
            | DeviceClass::Crusher => IndustryGroup::ConstructionEquipment,

            // Marine Equipment
            DeviceClass::SystemTools
            | DeviceClass::SafetySystems
            | DeviceClass::Gateway
            | DeviceClass::PowerManagementAndLightingSystems
            | DeviceClass::Steeringsystems
            | DeviceClass::NavigationSystems
            | DeviceClass::CommunicationsSystems
            | DeviceClass::InstrumentationOrGeneralSystems
            | DeviceClass::EnvironmentalSystems
            | DeviceClass::DeckCargoAndFishingEquipmentSystems => IndustryGroup::MarineEquipment,

            // Industrial Process Control
            DeviceClass::IndustrialProcessControlStationary => {
                IndustryGroup::IndustrialProcessControl
            }
        }
    }
}
