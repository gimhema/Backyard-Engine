#pragma once
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>
struct AllowConnectGame {
    uint32_t mid = 0;
    uint32_t sessionId = 0;
    uint32_t pid = 0;
    std::string accountId;
    std::string name;
    std::string connect_info;

    std::vector<uint8_t> serialize() const {
        std::vector<uint8_t> buffer;
        buffer.reserve(24 + accountId.size() + name.size() + connect_info.size());

        auto put_u32 = [&](uint32_t v) {
            uint8_t b[4];
            std::memcpy(b, &v, 4);
            buffer.insert(buffer.end(), b, b + 4);
            };
        auto put_str = [&](const std::string& s) {
            put_u32(static_cast<uint32_t>(s.size()));
            buffer.insert(buffer.end(),
                reinterpret_cast<const uint8_t*>(s.data()),
                reinterpret_cast<const uint8_t*>(s.data()) + s.size());
            };

        put_u32(mid);
        put_u32(sessionId);
        put_u32(pid);
        put_str(accountId);
        put_str(name);
        put_str(connect_info);
        return buffer;
    }

private:
    static bool try_deserialize_at(const std::vector<uint8_t>& buf, size_t start, AllowConnectGame& out) {
        size_t off = start;
        auto need = [&](size_t n) -> bool { return buf.size() - off >= n; };

        auto get_u32 = [&](uint32_t& v) -> bool {
            if (!need(4)) return false;
            std::memcpy(&v, buf.data() + off, 4);
            off += 4;
            return true;
            };

        auto get_str = [&](std::string& s) -> bool {
            uint32_t len = 0;
            if (!get_u32(len)) return false;
            constexpr uint32_t MAX_FIELD = 1u << 20; // 1MB 가드
            if (len > MAX_FIELD) return false;
            if (!need(len)) return false;
            s.assign(reinterpret_cast<const char*>(buf.data() + off), len);
            off += len;
            return true;
            };

        uint32_t mid = 0, sid = 0, pid = 0;
        if (!get_u32(mid) || !get_u32(sid) || !get_u32(pid)) return false;

        std::string a, n, c;
        if (!get_str(a) || !get_str(n) || !get_str(c)) return false;

        out.mid = mid;
        out.sessionId = sid;
        out.pid = pid;
        out.accountId = std::move(a);
        out.name = std::move(n);
        out.connect_info = std::move(c);
        return true;
    }

public:
    static AllowConnectGame deserialize(const std::vector<uint8_t>& buffer) {
        AllowConnectGame out;

        // (1) 프리픽스 추정: len == size  또는 len == size-4 → payload는 +4에서 시작
        size_t off0 = 0;
        if (buffer.size() >= 8) {
            uint32_t len;
            std::memcpy(&len, buffer.data(), 4);
            if (len == buffer.size() || len + 4 == buffer.size()) {
                off0 = 4;
            }
        }

        // (2) 우선 off0에서 시도
        if (try_deserialize_at(buffer, off0, out)) return out;

        // (3) 혹시 데이터 앞에 쓰레기/붙은 패킷이 있는 경우를 위한 백업 시도(옵션)
        if (off0 == 4 && try_deserialize_at(buffer, 0, out)) return out;

        // (4) 실패 시 기본값 반환(크래시 방지). 상위 로직에서 mid 체크 가드.
        return out;
    }
};
