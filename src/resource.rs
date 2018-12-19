pub type Url = String;

pub type FileSize = u64;

macro_rules! define_string_wrapper {
    ($($(#[$meta:meta])* $vis:vis $name:ident;)*) => { $(
        $(#[$meta])*
        #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
        $vis struct $name(String);

        impl $name {
            pub fn new(id: String) -> Self {
                $name(id)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    )* };
}

define_string_wrapper! {
    /// The unique identifier for a `Drive`,
    /// which can be get through `Client::get_drive`.
    pub DriveId;

    /// The unique identifier for a drive,
    /// which can be get through `Client::get_drive`.
    pub ItemId;

    /// An eTag for the state of an item.
    /// Used for avoid data transmission when a resource is not modified.
    ///
    /// The tag from `DriveItem::c_tag` is for the content of the item,
    /// while the one from `DriveItem::e_tag` is for the entire item (metadata + content).
    pub Tag;
}

/// The Drive resource.
///
/// # See also
/// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/drive?view=odsp-graph-online
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive { // TODO: Incomplete
    pub id: DriveId,
    // created_by: IdentitySet,
    // created_date_time: Timestamp,
    pub description: Option<String>,
    // drive_type: DriveType,
    pub items: Option<Vec<DriveItem>>,
    // last_modified_by: IdeneitySet,
    // last_modified_date_time: Timestamp,
    pub name: Option<String>,
    // owner: IdentitySet,
    // quota: Quota,
    // root: DriveItem,
    // sharepoint_ids: SharepointIds,
    pub special: Option<DriveItem>,
    // system: SystemFacet,
    pub web_url: Option<Url>,
}

/// The DriveItem resource.
///
/// # See also
/// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/driveitem?view=odsp-graph-online
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveItem { // TODO: Incomplete
    // Type specified fields

    // audio: Audio,
    // content: Stream,
    // file: File,
    // folder: Folder,
    // image: Image,
    // location: Option<GeoCoordinations>,
    // malware: Option<Malware>,
    // package: Package,
    // photo: Photo,
    // root: Root,
    // special_folder: SpecialFolder,
    // video: Video,

    // Drive item
    // c_tag: Option<Tag>,
    // deleted: Deleted,
    pub description: Option<String>,
    // pub file_system_info: FileSystemInfo,
    // publication: Option<PublicationFacet>,
    // remote_item: Option<RemoteItem>,
    // search_result: Option<SearchResult>,
    // shared: Shared,
    // sharepoint_ids: SharepointIds,
    pub size: FileSize,
    // web_dav_url: Url,

    // Relationships

    // activities: Vec<ItemActivity>,
    pub children: Option<Vec<DriveItem>>,
    // permissions: Vec<Permission>,
    // thumbnails: Vec<ThumbnailSet>,
    // versions: Vec<DriveItemVersion>,

    // Base item
    pub id: ItemId,
    // created_by: IdentitySet,
    // created_date_time: Timestamp,
    pub e_tag: Tag,
    // last_modified_by: IdentitySet,
    // last_modified_date_time: Timestamp,
    pub name: String,
    // parent_reference: ItemReference,
    pub web_url: Url,

    // Instance annotations
    #[serde(rename = "@microsoft.graph.downloadUrl")]
    pub download_url: Option<Url>,
}
